"""E2E tests for status_codes."""

from spikard.testing import TestClient
from app.main import (
    create_app_status_codes_19_413_payload_too_large,
    create_app_status_codes_200_ok_success,
    create_app_status_codes_201_created_resource_created,
    create_app_status_codes_202_accepted_request_accepted_for_processing,
    create_app_status_codes_204_no_content_success_with_no_body,
    create_app_status_codes_206_partial_content,
    create_app_status_codes_20_414_uri_too_long,
    create_app_status_codes_21_431_request_header_fields_too_large,
    create_app_status_codes_22_501_not_implemented,
    create_app_status_codes_23_503_service_unavailable,
    create_app_status_codes_301_moved_permanently_permanent_redirect,
    create_app_status_codes_302_found_temporary_redirect,
    create_app_status_codes_304_not_modified_cached_content_valid,
    create_app_status_codes_307_temporary_redirect_method_preserved,
    create_app_status_codes_400_bad_request_invalid_request,
    create_app_status_codes_401_unauthorized_missing_authentication,
    create_app_status_codes_403_forbidden_insufficient_permissions,
    create_app_status_codes_404_not_found_resource_not_found,
    create_app_status_codes_408_request_timeout,
    create_app_status_codes_422_unprocessable_entity_validation_error,
    create_app_status_codes_429_too_many_requests,
    create_app_status_codes_500_internal_server_error_server_error,
    create_app_status_codes_503_service_unavailable_server_overload,
)


async def test_408_request_timeout() -> None:
    """Tests 408 status code when request takes too long."""

    async with TestClient(create_app_status_codes_408_request_timeout()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"data": "large_data"}
        response = await client.post("/slow-endpoint", headers=headers, json=json_data)

        assert response.status_code == 408
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Request timeout"
        response_headers = response.headers
        assert response_headers.get("connection") == "close"


async def test_404_not_found_resource_not_found() -> None:
    """Tests 404 Not Found for non-existent resource."""

    async with TestClient(create_app_status_codes_404_not_found_resource_not_found()) as client:
        response = await client.get("/status-test/404")

        assert response.status_code == 404
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Item not found"


async def test_503_service_unavailable_server_overload() -> None:
    """Tests 503 Service Unavailable during maintenance or overload."""

    async with TestClient(create_app_status_codes_503_service_unavailable_server_overload()) as client:
        response = await client.get("/health")

        assert response.status_code == 503
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Service temporarily unavailable"
        response_headers = response.headers
        assert response_headers.get("retry-after") == "0"


async def test_422_unprocessable_entity_validation_error() -> None:
    """Tests 422 for validation errors (Pydantic)."""

    async with TestClient(create_app_status_codes_422_unprocessable_entity_validation_error()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"price": "not a number"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_302_found_temporary_redirect() -> None:
    """Tests 302 temporary redirect response."""

    async with TestClient(create_app_status_codes_302_found_temporary_redirect()) as client:
        response = await client.get("/temp-redirect")

        assert response.status_code == 302
        response_headers = response.headers
        assert response_headers.get("location") == "/target-path"


async def test_304_not_modified_cached_content_valid() -> None:
    """Tests 304 Not Modified for cached resources."""

    async with TestClient(create_app_status_codes_304_not_modified_cached_content_valid()) as client:
        headers = {
            "If-None-Match": '"abc123"',
        }
        response = await client.get("/status-test/304", headers=headers)

        assert response.status_code == 304


async def test_400_bad_request_invalid_request() -> None:
    """Tests 400 Bad Request for malformed request."""

    async with TestClient(create_app_status_codes_400_bad_request_invalid_request()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = "not valid json"
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 400
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Invalid request format"


async def test_22_501_not_implemented() -> None:
    """Unsupported HTTP method should return 501."""

    async with TestClient(create_app_status_codes_22_501_not_implemented()) as client:
        response = await client.trace("/data")

        assert response.status_code == 405


async def test_204_no_content_success_with_no_body() -> None:
    """Tests 204 No Content response for successful DELETE."""

    async with TestClient(create_app_status_codes_204_no_content_success_with_no_body()) as client:
        response = await client.delete("/status-test/204")

        assert response.status_code == 204


async def test_301_moved_permanently_permanent_redirect() -> None:
    """Tests 301 permanent redirect response."""

    async with TestClient(create_app_status_codes_301_moved_permanently_permanent_redirect()) as client:
        response = await client.get("/old-path")

        assert response.status_code == 301
        response_headers = response.headers
        assert response_headers.get("location") == "/new-path"


async def test_201_created_resource_created() -> None:
    """Tests 201 Created response for successful POST request."""

    async with TestClient(create_app_status_codes_201_created_resource_created()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "New Item"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 201
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "name" in response_data
        assert response_data["name"] == "New Item"


async def test_202_accepted_request_accepted_for_processing() -> None:
    """Tests 202 Accepted for async processing."""

    async with TestClient(create_app_status_codes_202_accepted_request_accepted_for_processing()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"task": "process_data"}
        response = await client.post("/tasks/", headers=headers, json=json_data)

        assert response.status_code == 202
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Task accepted for processing"
        assert "task_id" in response_data
        assert response_data["task_id"] == "abc123"


async def test_307_temporary_redirect_method_preserved() -> None:
    """Tests 307 temporary redirect with method preservation."""

    async with TestClient(create_app_status_codes_307_temporary_redirect_method_preserved()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {}
        response = await client.post("/redirect-post", headers=headers, json=json_data)

        assert response.status_code == 307
        response_data = response.json()
        response_headers = response.headers
        assert response_headers.get("location") == "/target-post"


async def test_500_internal_server_error_server_error() -> None:
    """Tests 500 Internal Server Error for unhandled exceptions."""

    async with TestClient(create_app_status_codes_500_internal_server_error_server_error()) as client:
        response = await client.get("/error")

        assert response.status_code == 500
        response_data = response.json()
        assert "type" in response_data
        assert response_data["type"] == "https://spikard.dev/errors/internal-server-error"
        assert "title" in response_data
        assert response_data["title"] == "Internal Server Error"
        assert "status" in response_data
        assert response_data["status"] == 500
        assert "detail" in response_data
        assert response_data["detail"] == "Internal server error"


async def test_20_414_uri_too_long() -> None:
    """Request with excessively long URI should return 414."""

    async with TestClient(create_app_status_codes_20_414_uri_too_long()) as client:
        response = await client.get("/data?skip_template_expansion=true")

        assert response.status_code == 200
        response_data = response.json()


async def test_401_unauthorized_missing_authentication() -> None:
    """Tests 401 Unauthorized when authentication is missing."""

    async with TestClient(create_app_status_codes_401_unauthorized_missing_authentication()) as client:
        response = await client.get("/users/me")

        assert response.status_code == 401
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Not authenticated"
        response_headers = response.headers
        assert response_headers.get("www-authenticate") == "Bearer"


async def test_23_503_service_unavailable() -> None:
    """Service temporarily unavailable should return 503 with Retry-After."""

    async with TestClient(create_app_status_codes_23_503_service_unavailable()) as client:
        response = await client.get("/data")

        assert response.status_code == 503
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Service Unavailable"
        assert "message" in response_data
        assert response_data["message"] == "The service is temporarily unavailable. Please try again later."
        response_headers = response.headers
        assert response_headers.get("retry-after") == "0"


async def test_19_413_payload_too_large() -> None:
    """Request with body exceeding max size should return 413."""

    async with TestClient(create_app_status_codes_19_413_payload_too_large()) as client:
        json_data = {"data": "{{ repeat 'x' 2000 times }}"}
        response = await client.post("/upload", json=json_data)

        assert response.status_code == 413
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Payload Too Large"
        assert "message" in response_data
        assert response_data["message"] == "Request body size exceeds maximum allowed size of 1024 bytes"


async def test_403_forbidden_insufficient_permissions() -> None:
    """Tests 403 Forbidden when user lacks permissions."""

    async with TestClient(create_app_status_codes_403_forbidden_insufficient_permissions()) as client:
        headers = {
            "Authorization": "Bearer valid_token",
        }
        response = await client.get("/admin/users", headers=headers)

        assert response.status_code == 403
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Not enough permissions"


async def test_21_431_request_header_fields_too_large() -> None:
    """Request with excessively large headers should return 431."""

    async with TestClient(create_app_status_codes_21_431_request_header_fields_too_large()) as client:
        headers = {
            "X-Large-Header": "{{ repeat 'x' 10000 times }}",
        }
        response = await client.get("/data", headers=headers)

        assert response.status_code == 431
        response_data = response.json()
        assert "error" in response_data
        assert response_data["error"] == "Request Header Fields Too Large"
        assert "message" in response_data
        assert response_data["message"] == "Request headers exceed maximum allowed size of 8192 bytes"


async def test_429_too_many_requests() -> None:
    """Tests 429 status code for rate limiting."""

    async with TestClient(create_app_status_codes_429_too_many_requests()) as client:
        response = await client.get("/api/resource")

        assert response.status_code == 429
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "Rate limit exceeded. Try again in 60 seconds."
        response_headers = response.headers
        assert response_headers.get("x-ratelimit-limit") == "100"
        assert response_headers.get("x-ratelimit-reset") == "1609459200"
        assert response_headers.get("retry-after") == "60"
        assert response_headers.get("x-ratelimit-remaining") == "0"


async def test_200_ok_success() -> None:
    """Tests standard 200 OK response for successful GET request."""

    async with TestClient(create_app_status_codes_200_ok_success()) as client:
        response = await client.get("/status-test/200")

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "name" in response_data
        assert response_data["name"] == "Item 1"


async def test_206_partial_content() -> None:
    """Tests 206 status code for range requests."""

    async with TestClient(create_app_status_codes_206_partial_content()) as client:
        headers = {
            "Range": "bytes=0-1023",
        }
        response = await client.get("/files/document.pdf", headers=headers)

        assert response.status_code == 206
        response_data = response.json()
        assert response_data == "binary_data_1024_bytes"
        response_headers = response.headers
        assert response_headers.get("content-type") == "application/pdf"
        assert response_headers.get("accept-ranges") == "bytes"
        assert response_headers.get("content-range") == "bytes 0-21/5000"

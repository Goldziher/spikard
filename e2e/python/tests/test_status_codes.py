"""E2E tests for status_codes."""

import pytest
from typing import Any

async def test_408_request_timeout(client: Any) -> None:
    """Tests 408 status code when request takes too long."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"data": "large_data"}
    response = await client.post("/slow-endpoint", headers=headers, json=json_data)

    assert response.status_code == 408
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request timeout"


async def test_404_not_found__resource_not_found(client: Any) -> None:
    """Tests 404 Not Found for non-existent resource."""
    response = await client.get("/status-test/404")

    assert response.status_code == 404
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Item not found"


async def test_503_service_unavailable__server_overload(client: Any) -> None:
    """Tests 503 Service Unavailable during maintenance or overload."""
    response = await client.get("/health")

    assert response.status_code == 503
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Service temporarily unavailable"


async def test_422_unprocessable_entity__validation_error(client: Any) -> None:
    """Tests 422 for validation errors (Pydantic)."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"price": "not a number"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == ""
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_302_found__temporary_redirect(client: Any) -> None:
    """Tests 302 temporary redirect response."""
    response = await client.get("/temp-redirect")

    assert response.status_code == 302


async def test_304_not_modified__cached_content_valid(client: Any) -> None:
    """Tests 304 Not Modified for cached resources."""
    headers = {
        "If-None-Match": "\"abc123\"",
    }
    response = await client.get("/status-test/304", headers=headers)

    assert response.status_code == 304


async def test_400_bad_request__invalid_request(client: Any) -> None:
    """Tests 400 Bad Request for malformed request."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = "not valid json"
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid request format"


async def test_22_501_not_implemented(client: Any) -> None:
    """Unsupported HTTP method should return 501."""
    response = await client.trace("/data")

    assert response.status_code == 501
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Not Implemented"
    assert "message" in response_data
    assert response_data["message"] == "The TRACE method is not supported by this server"


async def test_204_no_content__success_with_no_body(client: Any) -> None:
    """Tests 204 No Content response for successful DELETE."""
    response = await client.delete("/status-test/204")

    assert response.status_code == 204


async def test_301_moved_permanently__permanent_redirect(client: Any) -> None:
    """Tests 301 permanent redirect response."""
    response = await client.get("/old-path")

    assert response.status_code == 301


async def test_201_created__resource_created(client: Any) -> None:
    """Tests 201 Created response for successful POST request."""
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


async def test_202_accepted__request_accepted_for_processing(client: Any) -> None:
    """Tests 202 Accepted for async processing."""
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


async def test_307_temporary_redirect__method_preserved(client: Any) -> None:
    """Tests 307 temporary redirect with method preservation."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {}
    response = await client.post("/redirect-post", headers=headers, json=json_data)

    assert response.status_code == 307
    response_data = response.json()


async def test_500_internal_server_error__server_error(client: Any) -> None:
    """Tests 500 Internal Server Error for unhandled exceptions."""
    response = await client.get("/error")

    assert response.status_code == 500
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Internal server error"
    assert "status" in response_data
    assert response_data["status"] == 500
    assert "title" in response_data
    assert response_data["title"] == "Internal Server Error"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/internal-server-error"


async def test_20_414_uri_too_long(client: Any) -> None:
    """Request with excessively long URI should return 414."""
    response = await client.get("/data?{{ repeat 'param=value&' 300 times }}")

    assert response.status_code == 414
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "URI Too Long"
    assert "message" in response_data
    assert response_data["message"] == "Request URI exceeds maximum allowed length of 2048 characters"


async def test_401_unauthorized__missing_authentication(client: Any) -> None:
    """Tests 401 Unauthorized when authentication is missing."""
    response = await client.get("/users/me")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_23_503_service_unavailable(client: Any) -> None:
    """Service temporarily unavailable should return 503 with Retry-After."""
    response = await client.get("/data")

    assert response.status_code == 503
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Service Unavailable"
    assert "message" in response_data
    assert response_data["message"] == "The service is temporarily unavailable. Please try again later."


async def test_19_413_payload_too_large(client: Any) -> None:
    """Request with body exceeding max size should return 413."""
    json_data = {"data": "{{ repeat 'x' 2000 times }}"}
    response = await client.post("/upload", json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Payload Too Large"
    assert "message" in response_data
    assert response_data["message"] == "Request body size exceeds maximum allowed size of 1024 bytes"


async def test_403_forbidden__insufficient_permissions(client: Any) -> None:
    """Tests 403 Forbidden when user lacks permissions."""
    headers = {
        "Authorization": "Bearer valid_token",
    }
    response = await client.get("/admin/users", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not enough permissions"


async def test_21_431_request_header_fields_too_large(client: Any) -> None:
    """Request with excessively large headers should return 431."""
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


async def test_429_too_many_requests(client: Any) -> None:
    """Tests 429 status code for rate limiting."""
    response = await client.get("/api/resource")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Try again in 60 seconds."


async def test_200_ok__success(client: Any) -> None:
    """Tests standard 200 OK response for successful GET request."""
    response = await client.get("/status-test/200")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == 1
    assert "name" in response_data
    assert response_data["name"] == "Item 1"


async def test_206_partial_content(client: Any) -> None:
    """Tests 206 status code for range requests."""
    headers = {
        "Range": "bytes=0-1023",
    }
    response = await client.get("/files/document.pdf", headers=headers)

    assert response.status_code == 206
    response_data = response.json()
    assert response_data == "binary_data_1024_bytes"



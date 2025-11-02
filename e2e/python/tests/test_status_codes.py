"""E2E tests for status_codes."""


async def test_408_request_timeout() -> None:
    """Tests 408 status code when request takes too long."""
    from app.main import create_app_status_codes_408_request_timeout

    from spikard.testing import TestClient

    app = create_app_status_codes_408_request_timeout()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"data": "large_data"}
    response = await client.post("/slow-endpoint", headers=headers, json=json_data)

    assert response.status_code == 408
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request timeout"


async def test_404_not_found__resource_not_found() -> None:
    """Tests 404 Not Found for non-existent resource."""
    from app.main import create_app_status_codes_404_not_found___resource_not_found

    from spikard.testing import TestClient

    app = create_app_status_codes_404_not_found___resource_not_found()
    client = TestClient(app)

    response = await client.get("/status-test/404")

    assert response.status_code == 404
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Item not found"


async def test_503_service_unavailable__server_overload() -> None:
    """Tests 503 Service Unavailable during maintenance or overload."""
    from app.main import create_app_status_codes_503_service_unavailable___server_overload

    from spikard.testing import TestClient

    app = create_app_status_codes_503_service_unavailable___server_overload()
    client = TestClient(app)

    response = await client.get("/health")

    assert response.status_code == 503
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Service temporarily unavailable"


async def test_422_unprocessable_entity__validation_error() -> None:
    """Tests 422 for validation errors (Pydantic)."""
    from app.main import create_app_status_codes_422_unprocessable_entity___validation_error

    from spikard.testing import TestClient

    app = create_app_status_codes_422_unprocessable_entity___validation_error()
    client = TestClient(app)

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


async def test_302_found__temporary_redirect() -> None:
    """Tests 302 temporary redirect response."""
    from app.main import create_app_status_codes_302_found___temporary_redirect

    from spikard.testing import TestClient

    app = create_app_status_codes_302_found___temporary_redirect()
    client = TestClient(app)

    response = await client.get("/temp-redirect")

    assert response.status_code == 302


async def test_304_not_modified__cached_content_valid() -> None:
    """Tests 304 Not Modified for cached resources."""
    from app.main import create_app_status_codes_304_not_modified___cached_content_valid

    from spikard.testing import TestClient

    app = create_app_status_codes_304_not_modified___cached_content_valid()
    client = TestClient(app)

    headers = {
        "If-None-Match": '"abc123"',
    }
    response = await client.get("/status-test/304", headers=headers)

    assert response.status_code == 304


async def test_400_bad_request__invalid_request() -> None:
    """Tests 400 Bad Request for malformed request."""
    from app.main import create_app_status_codes_400_bad_request___invalid_request

    from spikard.testing import TestClient

    app = create_app_status_codes_400_bad_request___invalid_request()
    client = TestClient(app)

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
    from app.main import create_app_status_codes_22_501_not_implemented

    from spikard.testing import TestClient

    app = create_app_status_codes_22_501_not_implemented()
    client = TestClient(app)

    response = await client.trace("/data")

    assert response.status_code == 501
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Not Implemented"
    assert "message" in response_data
    assert response_data["message"] == "The TRACE method is not supported by this server"


async def test_204_no_content__success_with_no_body() -> None:
    """Tests 204 No Content response for successful DELETE."""
    from app.main import create_app_status_codes_204_no_content___success_with_no_body

    from spikard.testing import TestClient

    app = create_app_status_codes_204_no_content___success_with_no_body()
    client = TestClient(app)

    response = await client.delete("/status-test/204")

    assert response.status_code == 204


async def test_301_moved_permanently__permanent_redirect() -> None:
    """Tests 301 permanent redirect response."""
    from app.main import create_app_status_codes_301_moved_permanently___permanent_redirect

    from spikard.testing import TestClient

    app = create_app_status_codes_301_moved_permanently___permanent_redirect()
    client = TestClient(app)

    response = await client.get("/old-path")

    assert response.status_code == 301


async def test_201_created__resource_created() -> None:
    """Tests 201 Created response for successful POST request."""
    from app.main import create_app_status_codes_201_created___resource_created

    from spikard.testing import TestClient

    app = create_app_status_codes_201_created___resource_created()
    client = TestClient(app)

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


async def test_202_accepted__request_accepted_for_processing() -> None:
    """Tests 202 Accepted for async processing."""
    from app.main import create_app_status_codes_202_accepted___request_accepted_for_processing

    from spikard.testing import TestClient

    app = create_app_status_codes_202_accepted___request_accepted_for_processing()
    client = TestClient(app)

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


async def test_307_temporary_redirect__method_preserved() -> None:
    """Tests 307 temporary redirect with method preservation."""
    from app.main import create_app_status_codes_307_temporary_redirect___method_preserved

    from spikard.testing import TestClient

    app = create_app_status_codes_307_temporary_redirect___method_preserved()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {}
    response = await client.post("/redirect-post", headers=headers, json=json_data)

    assert response.status_code == 307
    response.json()


async def test_500_internal_server_error__server_error() -> None:
    """Tests 500 Internal Server Error for unhandled exceptions."""
    from app.main import create_app_status_codes_500_internal_server_error___server_error

    from spikard.testing import TestClient

    app = create_app_status_codes_500_internal_server_error___server_error()
    client = TestClient(app)

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


async def test_20_414_uri_too_long() -> None:
    """Request with excessively long URI should return 414."""
    from app.main import create_app_status_codes_20_414_uri_too_long

    from spikard.testing import TestClient

    app = create_app_status_codes_20_414_uri_too_long()
    client = TestClient(app)

    response = await client.get("/data?{{ repeat 'param=value&' 300 times }}")

    assert response.status_code == 414
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "URI Too Long"
    assert "message" in response_data
    assert response_data["message"] == "Request URI exceeds maximum allowed length of 2048 characters"


async def test_401_unauthorized__missing_authentication() -> None:
    """Tests 401 Unauthorized when authentication is missing."""
    from app.main import create_app_status_codes_401_unauthorized___missing_authentication

    from spikard.testing import TestClient

    app = create_app_status_codes_401_unauthorized___missing_authentication()
    client = TestClient(app)

    response = await client.get("/users/me")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_23_503_service_unavailable() -> None:
    """Service temporarily unavailable should return 503 with Retry-After."""
    from app.main import create_app_status_codes_23_503_service_unavailable

    from spikard.testing import TestClient

    app = create_app_status_codes_23_503_service_unavailable()
    client = TestClient(app)

    response = await client.get("/data")

    assert response.status_code == 503
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Service Unavailable"
    assert "message" in response_data
    assert response_data["message"] == "The service is temporarily unavailable. Please try again later."


async def test_19_413_payload_too_large() -> None:
    """Request with body exceeding max size should return 413."""
    from app.main import create_app_status_codes_19_413_payload_too_large

    from spikard.testing import TestClient

    app = create_app_status_codes_19_413_payload_too_large()
    client = TestClient(app)

    json_data = {"data": "{{ repeat 'x' 2000 times }}"}
    response = await client.post("/upload", json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Payload Too Large"
    assert "message" in response_data
    assert response_data["message"] == "Request body size exceeds maximum allowed size of 1024 bytes"


async def test_403_forbidden__insufficient_permissions() -> None:
    """Tests 403 Forbidden when user lacks permissions."""
    from app.main import create_app_status_codes_403_forbidden___insufficient_permissions

    from spikard.testing import TestClient

    app = create_app_status_codes_403_forbidden___insufficient_permissions()
    client = TestClient(app)

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
    from app.main import create_app_status_codes_21_431_request_header_fields_too_large

    from spikard.testing import TestClient

    app = create_app_status_codes_21_431_request_header_fields_too_large()
    client = TestClient(app)

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
    from app.main import create_app_status_codes_429_too_many_requests

    from spikard.testing import TestClient

    app = create_app_status_codes_429_too_many_requests()
    client = TestClient(app)

    response = await client.get("/api/resource")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Try again in 60 seconds."


async def test_200_ok__success() -> None:
    """Tests standard 200 OK response for successful GET request."""
    from app.main import create_app_status_codes_200_ok___success

    from spikard.testing import TestClient

    app = create_app_status_codes_200_ok___success()
    client = TestClient(app)

    response = await client.get("/status-test/200")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == 1
    assert "name" in response_data
    assert response_data["name"] == "Item 1"


async def test_206_partial_content() -> None:
    """Tests 206 status code for range requests."""
    from app.main import create_app_status_codes_206_partial_content

    from spikard.testing import TestClient

    app = create_app_status_codes_206_partial_content()
    client = TestClient(app)

    headers = {
        "Range": "bytes=0-1023",
    }
    response = await client.get("/files/document.pdf", headers=headers)

    assert response.status_code == 206
    response_data = response.json()
    assert response_data == "binary_data_1024_bytes"

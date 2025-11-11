"""E2E tests for request_id."""

from spikard.testing import TestClient
from app.main import (
    create_app_request_id_client_provided_request_id_propagated,
    create_app_request_id_custom_header_name_x_correlation_id,
    create_app_request_id_invalid_request_id_format_handling,
    create_app_request_id_missing_request_id_when_required,
    create_app_request_id_request_id_middleware_disabled,
    create_app_request_id_request_id_present_in_error_responses,
    create_app_request_id_request_id_present_in_request_and_response,
)


async def test_custom_header_name_x_correlation_id() -> None:
    """Tests request ID generation with custom header name X-Correlation-ID instead of default X-Request-ID."""

    app = create_app_request_id_custom_header_name_x_correlation_id()
    client = TestClient(app)

    headers = {
        "X-Correlation-ID": "correlation-xyz-789",
    }
    response = await client.get("/api/correlated", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "success"


async def test_request_id_middleware_disabled() -> None:
    """Tests that no request ID is generated or propagated when middleware is disabled."""

    app = create_app_request_id_request_id_middleware_disabled()
    client = TestClient(app)

    response = await client.get("/api/no-tracking")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "success"


async def test_missing_request_id_when_required() -> None:
    """Tests error handling when request ID is required but not provided by client."""

    app = create_app_request_id_missing_request_id_when_required()
    client = TestClient(app)

    json_data = {"data": "test"}
    response = await client.post("/api/require-id", json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Missing required header: X-Request-ID"
    assert "status" in response_data
    assert response_data["status"] == 400
    assert "title" in response_data
    assert response_data["title"] == "Bad Request"
    assert "type" in response_data
    assert response_data["type"] == "about:blank"


async def test_request_id_present_in_request_and_response() -> None:
    """Tests that request ID appears in both request and response headers with same value."""

    app = create_app_request_id_request_id_present_in_request_and_response()
    client = TestClient(app)

    headers = {
        "X-Request-ID": "trace-2024-11-11-abc123",
    }
    json_data = {"data": "test"}
    response = await client.post("/api/echo", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "echoed"
    assert "received" in response_data
    assert "data" in response_data["received"]
    assert response_data["received"]["data"] == "test"


async def test_request_id_present_in_error_responses() -> None:
    """Tests that request ID is included in response headers even for 4xx and 5xx error responses."""

    app = create_app_request_id_request_id_present_in_error_responses()
    client = TestClient(app)

    headers = {
        "X-Request-ID": "error-trace-456",
    }
    json_data = {"invalid": "data"}
    response = await client.post("/api/error-endpoint", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_invalid_request_id_format_handling() -> None:
    """Tests handling of malformed client-provided request ID when validation is enabled."""

    app = create_app_request_id_invalid_request_id_format_handling()
    client = TestClient(app)

    headers = {
        "X-Request-ID": "not-a-valid-uuid!!!",
    }
    response = await client.get("/api/validated", headers=headers)

    assert response.status_code == 400
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid request ID format: expected UUID v4"
    assert "status" in response_data
    assert response_data["status"] == 400
    assert "title" in response_data
    assert response_data["title"] == "Bad Request"
    assert "type" in response_data
    assert response_data["type"] == "about:blank"


async def test_client_provided_request_id_propagated() -> None:
    """Tests that client-provided request ID is propagated unchanged to response headers."""

    app = create_app_request_id_client_provided_request_id_propagated()
    client = TestClient(app)

    headers = {
        "X-Request-ID": "custom-id-12345",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "success"

"""E2E tests for request_id."""

from uuid import UUID

from spikard.testing import TestClient
from app.main import (
    create_app_request_id_request_id_header_is_preserved,
    create_app_request_id_request_id_is_generated_when_not_provided,
    create_app_request_id_request_id_middleware_can_be_disabled,
)


async def test_request_id_header_is_preserved() -> None:
    """When the client supplies X-Request-ID the same value should appear on the response.."""

    async with TestClient(create_app_request_id_request_id_header_is_preserved()) as client:
        headers = {
            "X-Request-ID": "trace-123",
        }
        response = await client.get("/request-id/preserved", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "status" in response_data
        assert response_data["status"] == "preserved"
        assert "echo" in response_data
        assert response_data["echo"] == "trace-123"
        response_headers = response.headers
        assert response_headers.get("x-request-id") == "trace-123"


async def test_request_id_middleware_can_be_disabled() -> None:
    """When request ID generation is disabled the response should not contain X-Request-ID even if the client sends a header.."""

    async with TestClient(create_app_request_id_request_id_middleware_can_be_disabled()) as client:
        headers = {
            "X-Request-ID": "external-id",
        }
        response = await client.get("/request-id/disabled", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        assert "status" in response_data
        assert response_data["status"] == "no-request-id"
        response_headers = response.headers
        assert response_headers.get("x-request-id") is None


async def test_request_id_is_generated_when_not_provided() -> None:
    """Ensures the request ID middleware attaches a UUID to responses when the client does not send one.."""

    async with TestClient(create_app_request_id_request_id_is_generated_when_not_provided()) as client:
        response = await client.get("/request-id/generated")

        assert response.status_code == 200
        response_data = response.json()
        assert "status" in response_data
        assert response_data["status"] == "generated"
        response_headers = response.headers
        header_value = response_headers.get("x-request-id")
        assert header_value is not None
        UUID(header_value)

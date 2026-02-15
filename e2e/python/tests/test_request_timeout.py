"""E2E tests for request_timeout."""

from spikard.testing import TestClient
from app.main import (
    create_app_request_timeout_request_completes_before_timeout,
    create_app_request_timeout_request_exceeds_timeout,
)


async def test_request_exceeds_timeout() -> None:
    """Simulates a handler that sleeps longer than the configured timeout to ensure a 408 response.."""

    async with TestClient(create_app_request_timeout_request_exceeds_timeout()) as client:
        response = await client.get("/timeouts/slow")

        assert response.status_code == 408


async def test_request_completes_before_timeout() -> None:
    """Simulated handler sleeps briefly and should complete before the timeout middleware fires.."""

    async with TestClient(create_app_request_timeout_request_completes_before_timeout()) as client:
        response = await client.get("/timeouts/fast")

        assert response.status_code == 200
        response_data = response.json()
        assert "status" in response_data
        assert response_data["status"] == "ok"
        assert "duration" in response_data
        assert response_data["duration"] == "fast"

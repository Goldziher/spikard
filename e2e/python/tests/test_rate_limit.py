"""E2E tests for rate_limit."""

import asyncio

from spikard.testing import TestClient
from app.main import (
    create_app_rate_limit_rate_limit_below_threshold_succeeds,
    create_app_rate_limit_rate_limit_exceeded_returns_429,
)


async def test_rate_limit_below_threshold_succeeds() -> None:
    """Verifies that requests below the configured rate limit are served normally.."""

    app = create_app_rate_limit_rate_limit_below_threshold_succeeds()
    client = TestClient(app)

    response = await client.get("/rate-limit/basic")

    assert response.status_code == 200
    response_data = response.json()
    assert "request" in response_data
    assert response_data["request"] == "under-limit"
    assert "status" in response_data
    assert response_data["status"] == "ok"


async def test_rate_limit_exceeded_returns_429() -> None:
    """Sends sequential requests until the configured limit is exceeded and validates the 429 response.."""

    app = create_app_rate_limit_rate_limit_exceeded_returns_429()
    client = TestClient(app)

    for _ in range(1):
        warmup_response = await client.get("/rate-limit/exceeded")
        assert warmup_response.status_code == 200
        await asyncio.sleep(0)
    response = await client.get("/rate-limit/exceeded")

    assert response.status_code == 429

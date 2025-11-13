"""E2E tests for body_limits."""

from spikard.testing import TestClient
from app.main import (
    create_app_body_limits_body_over_limit_returns_413,
    create_app_body_limits_body_under_limit_succeeds,
)


async def test_body_under_limit_succeeds() -> None:
    """Ensures requests smaller than the configured body limit are accepted.."""

    app = create_app_body_limits_body_under_limit_succeeds()
    client = TestClient(app)

    json_data = {"note": "small"}
    response = await client.post("/body-limit/under", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "accepted" in response_data
    assert response_data["accepted"] == True
    assert "note" in response_data
    assert response_data["note"] == "small"


async def test_body_over_limit_returns_413() -> None:
    """Requests that exceed the configured max body size should be rejected with Payload Too Large.."""

    app = create_app_body_limits_body_over_limit_returns_413()
    client = TestClient(app)

    json_data = {
        "note": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    }
    response = await client.post("/body-limit/over", json=json_data)

    assert response.status_code == 413

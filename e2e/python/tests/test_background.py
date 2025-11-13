"""E2E tests for background."""

from spikard.testing import TestClient
from app.main import (
    create_app_background_background_event_logging,
    create_app_background_background_event_logging_second_payload,
)


async def test_background_event_logging_second_payload() -> None:
    """Ensures background jobs handle different payloads."""

    app = create_app_background_background_event_logging_second_payload()
    client = TestClient(app)

    json_data = {"event": "beta"}
    response = await client.post("/background/events", json=json_data)

    assert response.status_code == 202
    state_response = await client.get("/background/events")
    assert state_response.status_code == 200
    assert state_response.json() == {"events": ["beta"]}


async def test_background_event_logging() -> None:
    """Enqueues a background job that appends the posted event to shared state."""

    app = create_app_background_background_event_logging()
    client = TestClient(app)

    json_data = {"event": "alpha"}
    response = await client.post("/background/events", json=json_data)

    assert response.status_code == 202
    state_response = await client.get("/background/events")
    assert state_response.status_code == 200
    assert state_response.json() == {"events": ["alpha"]}

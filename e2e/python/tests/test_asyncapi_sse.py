"""AsyncAPI SSE tests."""

import json

from spikard.testing import TestClient
from app.main import (
    create_app_sse_notifications,
)


async def test_sse_notifications() -> None:
    """SSE channel test for /notifications."""
    app = create_app_sse_notifications()
    client = TestClient(app)
    response = await client.get("/notifications")
    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:] for chunk in normalized.split("\n\n") if chunk.startswith("data:")]
    expected = [
        '{"level":"example_level","message":"example_message","source":"example_source","timestamp":"2024-01-15T10:30:00Z","type":"system_alert"}',
        '{"body":"example_body","priority":"example_priority","timestamp":"2024-01-15T10:30:00Z","title":"example_title","type":"user_notification","userId":"example_userId"}',
        '{"message":"example_message","metadata":{},"service":"example_service","status":"example_status","timestamp":"2024-01-15T10:30:00Z","type":"status_update"}',
    ]
    assert len(events) == len(expected)
    for payload, expected_json in zip(events, expected):
        assert json.loads(payload.strip()) == json.loads(expected_json)

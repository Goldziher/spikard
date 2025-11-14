"""AsyncAPI SSE tests using real server and httpx-sse library."""

import json

import pytest
from httpx_sse import aconnect_sse


@pytest.mark.asyncio
async def test_sse_notifications(http_client) -> None:
    """SSE channel test for /notifications."""
    expected = [
        {
            "level": "example_level",
            "message": "example_message",
            "source": "example_source",
            "timestamp": "2024-01-15T10:30:00Z",
            "type": "system_alert",
        },
        {
            "body": "example_body",
            "priority": "example_priority",
            "timestamp": "2024-01-15T10:30:00Z",
            "title": "example_title",
            "type": "user_notification",
            "userId": "example_userId",
        },
        {
            "message": "example_message",
            "metadata": {},
            "service": "example_service",
            "status": "example_status",
            "timestamp": "2024-01-15T10:30:00Z",
            "type": "status_update",
        },
    ]

    events = []
    async with aconnect_sse(http_client, "GET", "/notifications") as event_source:
        async for event in event_source.aiter_sse():
            events.append(json.loads(event.data))
            if len(events) >= len(expected):
                break

    assert len(events) == len(expected)
    for received, expected_data in zip(events, expected):
        assert received == expected_data

"""E2E tests for request_timeout."""

from spikard.testing import TestClient
from app.main import (
    create_app_request_timeout_concurrent_requests_timing_out,
    create_app_request_timeout_custom_long_timeout,
    create_app_request_timeout_custom_short_timeout,
    create_app_request_timeout_default_timeout_behavior,
    create_app_request_timeout_disabled_timeout,
    create_app_request_timeout_per_route_timeout_override,
    create_app_request_timeout_request_at_exact_timeout_boundary,
    create_app_request_timeout_request_completes_within_timeout,
    create_app_request_timeout_request_exceeds_timeout,
    create_app_request_timeout_streaming_response_timeout,
    create_app_request_timeout_timeout_during_slow_request_body_read,
    create_app_request_timeout_timeout_with_partial_response,
)


async def test_per_route_timeout_override() -> None:
    """Tests route-specific timeout overriding global timeout."""

    app = create_app_request_timeout_per_route_timeout_override()
    client = TestClient(app)

    response = await client.get("/api/custom-route")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 5 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_custom_long_timeout() -> None:
    """Tests extended 60 second timeout for long-running operations."""

    app = create_app_request_timeout_custom_long_timeout()
    client = TestClient(app)

    json_data = {"items": [1, 2, 3]}
    response = await client.post("/api/batch-process", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "delay_ms" in response_data
    assert response_data["delay_ms"] == 45000
    assert "items_processed" in response_data
    assert response_data["items_processed"] == 3
    assert "message" in response_data
    assert response_data["message"] == "batch processed"


async def test_disabled_timeout() -> None:
    """Tests that zero/null timeout disables timeout enforcement."""

    app = create_app_request_timeout_disabled_timeout()
    client = TestClient(app)

    response = await client.get("/api/no-timeout")

    assert response.status_code == 200
    response_data = response.json()
    assert "delay_ms" in response_data
    assert response_data["delay_ms"] == 120000
    assert "message" in response_data
    assert response_data["message"] == "completed without timeout"


async def test_request_at_exact_timeout_boundary() -> None:
    """Tests behavior when request completes at the exact timeout threshold."""

    app = create_app_request_timeout_request_at_exact_timeout_boundary()
    client = TestClient(app)

    response = await client.get("/api/boundary")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 3 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_custom_short_timeout() -> None:
    """Tests aggressive 1 second timeout for fast endpoints."""

    app = create_app_request_timeout_custom_short_timeout()
    client = TestClient(app)

    response = await client.get("/api/fast-only")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 1 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_timeout_during_slow_request_body_read() -> None:
    """Tests timeout when client sends request body too slowly."""

    app = create_app_request_timeout_timeout_during_slow_request_body_read()
    client = TestClient(app)

    headers = {
        "Content-Length": "1048576",
    }
    json_data = {"data": "large payload"}
    response = await client.post("/api/upload", headers=headers, json=json_data)

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 5 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_request_completes_within_timeout() -> None:
    """Tests successful response when request completes well within configured timeout."""

    app = create_app_request_timeout_request_completes_within_timeout()
    client = TestClient(app)

    response = await client.get("/api/fast")

    assert response.status_code == 200
    response_data = response.json()
    assert "delay_ms" in response_data
    assert response_data["delay_ms"] == 1000
    assert "message" in response_data
    assert response_data["message"] == "success"


async def test_concurrent_requests_timing_out() -> None:
    """Tests multiple concurrent requests all exceeding timeout."""

    app = create_app_request_timeout_concurrent_requests_timing_out()
    client = TestClient(app)

    response = await client.get("/api/slow-concurrent")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 2 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_default_timeout_behavior() -> None:
    """Tests default 30 second timeout when no timeout is explicitly configured."""

    app = create_app_request_timeout_default_timeout_behavior()
    client = TestClient(app)

    response = await client.get("/api/default")

    assert response.status_code == 200
    response_data = response.json()
    assert "delay_ms" in response_data
    assert response_data["delay_ms"] == 2000
    assert "message" in response_data
    assert response_data["message"] == "success"


async def test_streaming_response_timeout() -> None:
    """Tests timeout during chunked/streaming response."""

    app = create_app_request_timeout_streaming_response_timeout()
    client = TestClient(app)

    response = await client.get("/api/stream")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 3 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_request_exceeds_timeout() -> None:
    """Tests 504 response when request processing exceeds configured timeout."""

    app = create_app_request_timeout_request_exceeds_timeout()
    client = TestClient(app)

    response = await client.get("/api/slow")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 2 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"


async def test_timeout_with_partial_response() -> None:
    """Tests timeout when some response data has already been sent."""

    app = create_app_request_timeout_timeout_with_partial_response()
    client = TestClient(app)

    response = await client.get("/api/partial")

    assert response.status_code == 504
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request processing exceeded the configured timeout of 2 seconds"
    assert "status" in response_data
    assert response_data["status"] == 504
    assert "title" in response_data
    assert response_data["title"] == "Gateway Timeout"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/request-timeout"

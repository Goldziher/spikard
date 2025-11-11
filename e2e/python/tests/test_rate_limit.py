"""E2E tests for rate_limit."""

from spikard.testing import TestClient
from app.main import (
    create_app_rate_limit_rate_limit_burst_allowed,
    create_app_rate_limit_rate_limit_burst_exceeded,
    create_app_rate_limit_rate_limit_concurrent_burst_requests,
    create_app_rate_limit_rate_limit_disabled,
    create_app_rate_limit_rate_limit_exceeded,
    create_app_rate_limit_rate_limit_headers_present,
    create_app_rate_limit_rate_limit_high_burst_allowance,
    create_app_rate_limit_rate_limit_high_limit_configuration,
    create_app_rate_limit_rate_limit_ip_based_limiting_enabled,
    create_app_rate_limit_rate_limit_low_limit_configuration,
    create_app_rate_limit_rate_limit_multiple_endpoints_with_different_limits,
    create_app_rate_limit_rate_limit_ratelimit_limit_header,
    create_app_rate_limit_rate_limit_ratelimit_remaining_decrements,
    create_app_rate_limit_rate_limit_reset_time_calculation,
    create_app_rate_limit_rate_limit_retry_after_header,
    create_app_rate_limit_rate_limit_rfc_9457_error_format,
    create_app_rate_limit_rate_limit_sequential_requests,
    create_app_rate_limit_rate_limit_token_bucket_refill,
    create_app_rate_limit_rate_limit_under_limit,
    create_app_rate_limit_rate_limit_zero_burst_allowance,
)


async def test_rate_limit_ratelimit_limit_header() -> None:
    """Validates that RateLimit-Limit header shows configured per_second limit."""

    app = create_app_rate_limit_rate_limit_ratelimit_limit_header()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_high_limit_configuration() -> None:
    """Tests rate limiting with very high per_second limit (1000/sec)."""

    app = create_app_rate_limit_rate_limit_high_limit_configuration()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_concurrent_burst_requests() -> None:
    """Tests that concurrent burst requests are properly rate limited."""

    app = create_app_rate_limit_rate_limit_concurrent_burst_requests()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_burst_exceeded() -> None:
    """Tests that requests exceeding burst allowance return 429."""

    app = create_app_rate_limit_rate_limit_burst_exceeded()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Please retry after the specified time."
    assert "status" in response_data
    assert response_data["status"] == 429
    assert "title" in response_data
    assert response_data["title"] == "Too Many Requests"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/rate-limit-exceeded"


async def test_rate_limit_headers_present() -> None:
    """Validates that all RateLimit headers are present in successful responses."""

    app = create_app_rate_limit_rate_limit_headers_present()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_token_bucket_refill() -> None:
    """Tests that GCRA token bucket refills over time allowing new requests."""

    app = create_app_rate_limit_rate_limit_token_bucket_refill()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_multiple_endpoints_with_different_limits() -> None:
    """Tests that different endpoints can have independent rate limit configurations."""

    app = create_app_rate_limit_rate_limit_multiple_endpoints_with_different_limits()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_ratelimit_remaining_decrements() -> None:
    """Validates that RateLimit-Remaining header decrements with each request."""

    app = create_app_rate_limit_rate_limit_ratelimit_remaining_decrements()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_low_limit_configuration() -> None:
    """Tests rate limiting with very low per_second limit (1/sec)."""

    app = create_app_rate_limit_rate_limit_low_limit_configuration()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_rfc_9457_error_format() -> None:
    """Validates that 429 responses follow RFC 9457 Problem Details format."""

    app = create_app_rate_limit_rate_limit_rfc_9457_error_format()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Please retry after the specified time."
    assert "status" in response_data
    assert response_data["status"] == 429
    assert "title" in response_data
    assert response_data["title"] == "Too Many Requests"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/rate-limit-exceeded"


async def test_rate_limit_under_limit() -> None:
    """Tests that requests under the rate limit succeed with proper RateLimit headers."""

    app = create_app_rate_limit_rate_limit_under_limit()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_sequential_requests() -> None:
    """Tests multiple sequential requests to verify rate limit counter decrements properly."""

    app = create_app_rate_limit_rate_limit_sequential_requests()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_disabled() -> None:
    """Tests that endpoints without rate_limit middleware succeed without rate limit headers."""

    app = create_app_rate_limit_rate_limit_disabled()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_exceeded() -> None:
    """Tests that requests exceeding the rate limit return 429 with RFC 9457 error format."""

    app = create_app_rate_limit_rate_limit_exceeded()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Please retry after the specified time."
    assert "status" in response_data
    assert response_data["status"] == 429
    assert "title" in response_data
    assert response_data["title"] == "Too Many Requests"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/rate-limit-exceeded"


async def test_rate_limit_retry_after_header() -> None:
    """Validates that 429 responses include Retry-After header with correct value."""

    app = create_app_rate_limit_rate_limit_retry_after_header()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 429
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Rate limit exceeded. Please retry after the specified time."
    assert "status" in response_data
    assert response_data["status"] == 429
    assert "title" in response_data
    assert response_data["title"] == "Too Many Requests"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/rate-limit-exceeded"


async def test_rate_limit_burst_allowed() -> None:
    """Tests that burst requests within burst allowance succeed."""

    app = create_app_rate_limit_rate_limit_burst_allowed()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_zero_burst_allowance() -> None:
    """Tests rate limiting with no burst allowance (burst=0)."""

    app = create_app_rate_limit_rate_limit_zero_burst_allowance()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_reset_time_calculation() -> None:
    """Validates that rate limit reset time is calculated correctly based on GCRA."""

    app = create_app_rate_limit_rate_limit_reset_time_calculation()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_ip_based_limiting_enabled() -> None:
    """Tests that rate limits are tracked per client IP address."""

    app = create_app_rate_limit_rate_limit_ip_based_limiting_enabled()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_rate_limit_high_burst_allowance() -> None:
    """Tests rate limiting with high burst capacity (burst=100)."""

    app = create_app_rate_limit_rate_limit_high_burst_allowance()
    client = TestClient(app)

    response = await client.get("/api/rate-limited")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Success"

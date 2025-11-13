"""E2E tests for compression."""

from spikard.testing import TestClient
from app.main import (
    create_app_compression_compression_gzip_applied,
    create_app_compression_compression_payload_below_min_size_is_not_compressed,
)


async def test_compression_payload_below_min_size_is_not_compressed() -> None:
    """Ensures responses smaller than the configured min_size are sent uncompressed even when the client sends Accept-Encoding.."""

    app = create_app_compression_compression_payload_below_min_size_is_not_compressed()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/compression/skip", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Small payload"
    assert "payload" in response_data
    assert response_data["payload"] == "tiny"
    response_headers = response.headers
    assert response_headers.get("content-encoding") is None


async def test_compression_gzip_applied() -> None:
    """Serves a JSON payload compressed with gzip when the client advertises support.."""

    app = create_app_compression_compression_gzip_applied()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/compression/gzip", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Compressed payload"
    assert "payload" in response_data
    assert (
        response_data["payload"]
        == "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    )
    response_headers = response.headers
    assert response_headers.get("content-encoding") == "gzip"
    assert response_headers.get("vary") == "Accept-Encoding"

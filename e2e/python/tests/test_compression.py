"""E2E tests for compression."""

from spikard.testing import TestClient
from app.main import (
    create_app_compression_compression_brotli_applied,
    create_app_compression_compression_client_prefers_brotli,
    create_app_compression_compression_client_prefers_gzip,
    create_app_compression_compression_gzip_applied,
    create_app_compression_compression_high_compression_level,
    create_app_compression_compression_large_json_response,
    create_app_compression_compression_low_compression_level,
    create_app_compression_compression_minimum_size_threshold,
    create_app_compression_compression_multiple_acceptable_encodings,
    create_app_compression_compression_wildcard_encoding,
)


async def test_compression_wildcard_encoding() -> None:
    """Tests that server applies compression when client sends Accept-Encoding: * (any encoding acceptable)."""

    app = create_app_compression_compression_wildcard_encoding()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "*",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed with server's preferred algorithm"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_gzip_applied() -> None:
    """Tests gzip compression when client supports it via Accept-Encoding header."""

    app = create_app_compression_compression_gzip_applied()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload that benefits from compression"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_brotli_applied() -> None:
    """Tests brotli compression when client supports it via Accept-Encoding header."""

    app = create_app_compression_compression_brotli_applied()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "br",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload that benefits from compression"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_minimum_size_threshold() -> None:
    """Tests that compression is only applied when response size exceeds configured threshold."""

    app = create_app_compression_compression_minimum_size_threshold()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload exceeding minimum threshold"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_high_compression_level() -> None:
    """Tests high compression level (9) for maximum size reduction at cost of CPU time."""

    app = create_app_compression_compression_high_compression_level()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed at maximum level"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_client_prefers_gzip() -> None:
    """Tests content negotiation when client prefers gzip over brotli using quality values."""

    app = create_app_compression_compression_client_prefers_gzip()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip, br;q=0.8",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed with gzip"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_client_prefers_brotli() -> None:
    """Tests content negotiation when client prefers brotli over gzip using quality values."""

    app = create_app_compression_compression_client_prefers_brotli()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "br, gzip;q=0.8",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed with brotli"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_large_json_response() -> None:
    """Tests compression of large JSON response with high compression ratio due to text redundancy."""

    app = create_app_compression_compression_large_json_response()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "br",
    }
    response = await client.get("/api/records", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "records" in response_data
    assert response_data["records"] == "Array of 1000+ JSON objects with repeated structure"
    assert "total" in response_data
    assert response_data["total"] == 1000


async def test_compression_low_compression_level() -> None:
    """Tests low compression level (1) for faster processing with modest size reduction."""

    app = create_app_compression_compression_low_compression_level()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed at low level"
    assert "message" in response_data
    assert response_data["message"] == "Success"


async def test_compression_multiple_acceptable_encodings() -> None:
    """Tests that server selects best available algorithm when client accepts multiple encodings."""

    app = create_app_compression_compression_multiple_acceptable_encodings()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip, deflate, br",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "Large payload compressed with best algorithm"
    assert "message" in response_data
    assert response_data["message"] == "Success"

"""E2E tests for body_size_limit."""

from spikard.testing import TestClient
from app.main import (
    create_app_body_size_limit_body_size_at_exact_boundary,
    create_app_body_size_limit_body_size_exceeds_limit,
    create_app_body_size_limit_body_size_within_limit,
    create_app_body_size_limit_form_data_exceeds_limit,
    create_app_body_size_limit_json_body_exceeds_limit,
    create_app_body_size_limit_large_limit_for_file_upload_endpoint,
    create_app_body_size_limit_multipart_file_upload_exceeds_limit,
    create_app_body_size_limit_per_route_limit_override,
    create_app_body_size_limit_small_limit_for_text_endpoint,
    create_app_body_size_limit_streaming_body_exceeds_limit,
)


async def test_multipart_file_upload_exceeds_limit() -> None:
    """Multipart file upload exceeding limit returns 413."""

    app = create_app_body_size_limit_multipart_file_upload_exceeds_limit()
    client = TestClient(app)

    headers = {
        "Content-Type": "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
        "Content-Length": "2097152",
    }
    raw_body = {"simulated_size_bytes": 2097152}
    response = await client.post("/api/upload/avatar", headers=headers, data=raw_body)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (2097152 bytes) exceeds maximum allowed size (1048576 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_form_data_exceeds_limit() -> None:
    """URL-encoded form data exceeding limit returns 413."""

    app = create_app_body_size_limit_form_data_exceeds_limit()
    client = TestClient(app)

    headers = {
        "Content-Length": "4096",
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"simulated_size_bytes": 4096}
    response = await client.post("/api/form", headers=headers, json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (4096 bytes) exceeds maximum allowed size (2048 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_json_body_exceeds_limit() -> None:
    """Large JSON payload exceeding limit returns 413."""

    app = create_app_body_size_limit_json_body_exceeds_limit()
    client = TestClient(app)

    headers = {
        "Content-Length": "10240",
        "Content-Type": "application/json",
    }
    json_data = {"simulated_size_bytes": 10240}
    response = await client.post("/api/json", headers=headers, json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (10240 bytes) exceeds maximum allowed size (5120 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_body_size_at_exact_boundary() -> None:
    """Request body at exact size limit should succeed with 200."""

    app = create_app_body_size_limit_body_size_at_exact_boundary()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
        "Content-Length": "10240",
    }
    json_data = {"content": {"data": "test payload at exact boundary"}, "simulated_size_bytes": 10240}
    response = await client.post("/api/data", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "bytes_received" in response_data
    assert response_data["bytes_received"] == 10240
    assert "message" in response_data
    assert response_data["message"] == "Data received successfully"


async def test_large_limit_for_file_upload_endpoint() -> None:
    """Large 100MB limit appropriate for file upload endpoints."""

    app = create_app_body_size_limit_large_limit_for_file_upload_endpoint()
    client = TestClient(app)

    headers = {
        "Content-Type": "multipart/form-data; boundary=----WebKitFormBoundary",
        "Content-Length": "52428800",
    }
    raw_body = {"content": {"file": "large_file.bin"}, "simulated_size_bytes": 52428800}
    response = await client.post("/api/upload", headers=headers, data=raw_body)

    assert response.status_code == 200
    response_data = response.json()
    assert "bytes_received" in response_data
    assert response_data["bytes_received"] == 52428800
    assert "message" in response_data
    assert response_data["message"] == "File uploaded successfully"


async def test_per_route_limit_override() -> None:
    """Different limits per route: small for text, large for upload."""

    app = create_app_body_size_limit_per_route_limit_override()
    client = TestClient(app)

    headers = {
        "Content-Type": "multipart/form-data; boundary=----WebKitFormBoundary",
        "Content-Length": "10485760",
    }
    raw_body = {"content": {"file": "medium_file.bin"}, "simulated_size_bytes": 10485760}
    response = await client.post("/api/upload", headers=headers, data=raw_body)

    assert response.status_code == 200
    response_data = response.json()
    assert "bytes_received" in response_data
    assert response_data["bytes_received"] == 10485760
    assert "message" in response_data
    assert response_data["message"] == "File uploaded successfully"


async def test_body_size_exceeds_limit() -> None:
    """Tests 413 response when request body exceeds configured size limit."""

    app = create_app_body_size_limit_body_size_exceeds_limit()
    client = TestClient(app)

    headers = {
        "Content-Length": "20480",
        "Content-Type": "application/json",
    }
    json_data = {"simulated_size_bytes": 20480}
    response = await client.post("/api/data", headers=headers, json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (20480 bytes) exceeds maximum allowed size (10240 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_small_limit_for_text_endpoint() -> None:
    """Small 1KB limit appropriate for text-only endpoints."""

    app = create_app_body_size_limit_small_limit_for_text_endpoint()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
        "Content-Length": "2048",
    }
    json_data = {"simulated_size_bytes": 2048}
    response = await client.post("/api/text", headers=headers, json=json_data)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (2048 bytes) exceeds maximum allowed size (1024 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_streaming_body_exceeds_limit() -> None:
    """Streaming request body exceeding limit returns 413 during stream processing."""

    app = create_app_body_size_limit_streaming_body_exceeds_limit()
    client = TestClient(app)

    headers = {
        "Transfer-Encoding": "chunked",
        "Content-Type": "application/octet-stream",
    }
    raw_body = {"simulated_size_bytes": 16384}
    response = await client.post("/api/stream", headers=headers, data=raw_body)

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Request body size (16384 bytes) exceeds maximum allowed size (8192 bytes)"
    assert "status" in response_data
    assert response_data["status"] == 413
    assert "title" in response_data
    assert response_data["title"] == "Payload Too Large"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/payload-too-large"


async def test_body_size_within_limit() -> None:
    """Request body within configured limit should succeed with 200."""

    app = create_app_body_size_limit_body_size_within_limit()
    client = TestClient(app)

    headers = {
        "Content-Length": "1024",
        "Content-Type": "application/json",
    }
    json_data = {"content": {"data": "test payload within limit"}, "simulated_size_bytes": 1024}
    response = await client.post("/api/data", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "bytes_received" in response_data
    assert response_data["bytes_received"] == 1024
    assert "message" in response_data
    assert response_data["message"] == "Data received successfully"

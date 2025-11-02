"""E2E tests for content_types."""

import pytest
from typing import Any

async def test_415_unsupported_media_type() -> None:
    """Tests rejection of unsupported content type."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_415_unsupported_media_type

    app = create_app_content_types_415_unsupported_media_type()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/xml",
    }
    json_data = "<?xml version=\"1.0\"?><item><name>Item</name></item>"
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Unsupported media type"


async def test_xml_response_application_xml() -> None:
    """Tests XML response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_xml_response_application_xml

    app = create_app_content_types_xml_response_application_xml()
    client = TestClient(app)

    response = await client.get("/xml")

    assert response.status_code == 200
    response_data = response.json()


async def test_14_content_type_case_insensitive() -> None:
    """Content-Type header should be case-insensitive."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_14_content_type_case_insensitive

    app = create_app_content_types_14_content_type_case_insensitive()
    client = TestClient(app)

    headers = {
        "Content-Type": "APPLICATION/JSON",
    }
    json_data = {"name": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "test"


async def test_json_with_utf_8_charset() -> None:
    """Tests JSON response with explicit UTF-8 charset."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_json_with_utf_8_charset

    app = create_app_content_types_json_with_utf_8_charset()
    client = TestClient(app)

    response = await client.get("/items/unicode")

    assert response.status_code == 200
    response_data = response.json()


async def test_16_text_plain_not_accepted() -> None:
    """text/plain content-type should be rejected when JSON is expected."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_16_text_plain_not_accepted

    app = create_app_content_types_16_text_plain_not_accepted()
    client = TestClient(app)

    headers = {
        "Content-Type": "text/plain",
    }
    json_data = "{\"data\": \"value\"}"
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Unsupported Media Type. Expected application/json"


async def test_pdf_response_application_pdf() -> None:
    """Tests PDF file response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_pdf_response_application_pdf

    app = create_app_content_types_pdf_response_application_pdf()
    client = TestClient(app)

    response = await client.get("/download/document.pdf")

    assert response.status_code == 200
    response_data = response.json()


async def test_20_content_length_mismatch() -> None:
    """Content-Length header mismatch with actual body size should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_20_content_length_mismatch

    app = create_app_content_types_20_content_length_mismatch()
    client = TestClient(app)

    headers = {
        "Content-Length": "100",
        "Content-Type": "application/json",
    }
    json_data = {"value": "short"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Content-Length header does not match actual body size"


async def test_17_vendor_json_accepted() -> None:
    """Vendor-specific JSON content-type should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_17_vendor_json_accepted

    app = create_app_content_types_17_vendor_json_accepted()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/vnd.api+json",
    }
    json_data = {"data": "value"}
    response = await client.post("/api/v1/resource", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "value"


async def test_13_json_with_charset_utf16() -> None:
    """JSON with UTF-16 charset should be rejected (UTF-8 only)."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_13_json_with_charset_utf16

    app = create_app_content_types_13_json_with_charset_utf16()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json; charset=utf-16",
    }
    json_data = {"value": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."


async def test_json_response_application_json() -> None:
    """Tests JSON response with correct Content-Type header."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_json_response_application_json

    app = create_app_content_types_json_response_application_json()
    client = TestClient(app)

    response = await client.get("/items/json")

    assert response.status_code == 200
    response_data = response.json()


async def test_15_multipart_boundary_required() -> None:
    """Multipart content-type without boundary parameter should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_15_multipart_boundary_required

    app = create_app_content_types_15_multipart_boundary_required()
    client = TestClient(app)

    headers = {
        "Content-Type": "multipart/form-data",
    }
    response = await client.post("/upload", headers=headers)

    assert response.status_code == 400
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "multipart/form-data requires 'boundary' parameter"


async def test_content_negotiation_accept_header() -> None:
    """Tests content negotiation based on Accept header."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_content_negotiation_accept_header

    app = create_app_content_types_content_negotiation_accept_header()
    client = TestClient(app)

    headers = {
        "Accept": "application/json",
    }
    response = await client.get("/accept-test/1", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_html_response_text_html() -> None:
    """Tests HTML response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_html_response_text_html

    app = create_app_content_types_html_response_text_html()
    client = TestClient(app)

    response = await client.get("/html")

    assert response.status_code == 200
    response_data = response.json()


async def test_jpeg_image_response_image_jpeg() -> None:
    """Tests JPEG image response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_jpeg_image_response_image_jpeg

    app = create_app_content_types_jpeg_image_response_image_jpeg()
    client = TestClient(app)

    response = await client.get("/images/photo.jpg")

    assert response.status_code == 200
    response_data = response.json()


async def test_19_missing_content_type_default_json() -> None:
    """Missing Content-Type header should default to JSON when body is present."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_19_missing_content_type_default_json

    app = create_app_content_types_19_missing_content_type_default_json()
    client = TestClient(app)

    json_data = {"name": "test"}
    response = await client.post("/data", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "test"


async def test_png_image_response_image_png() -> None:
    """Tests PNG image response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_png_image_response_image_png

    app = create_app_content_types_png_image_response_image_png()
    client = TestClient(app)

    response = await client.get("/images/logo.png")

    assert response.status_code == 200
    response_data = response.json()


async def test_plain_text_response_text_plain() -> None:
    """Tests plain text response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_plain_text_response_text_plain

    app = create_app_content_types_plain_text_response_text_plain()
    client = TestClient(app)

    response = await client.get("/text")

    assert response.status_code == 200
    response_data = response.json()


async def test_18_content_type_with_multiple_params() -> None:
    """Content-Type with multiple parameters should be parsed correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_18_content_type_with_multiple_params

    app = create_app_content_types_18_content_type_with_multiple_params()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json; charset=utf-8; boundary=something",
    }
    json_data = {"value": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == "test"


async def test_csv_response_text_csv() -> None:
    """Tests CSV file response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_csv_response_text_csv

    app = create_app_content_types_csv_response_text_csv()
    client = TestClient(app)

    response = await client.get("/export/data.csv")

    assert response.status_code == 200
    response_data = response.json()


async def test_binary_response_application_octet_stream() -> None:
    """Tests binary data response."""
    from spikard.testing import TestClient
    from app.main import create_app_content_types_binary_response_application_octet_stream

    app = create_app_content_types_binary_response_application_octet_stream()
    client = TestClient(app)

    response = await client.get("/download/file.bin")

    assert response.status_code == 200
    response_data = response.json()



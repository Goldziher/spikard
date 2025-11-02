"""E2E tests for content_types."""

import pytest
from typing import Any

async def test_415_unsupported_media_type(client: Any) -> None:
    """Tests rejection of unsupported content type."""
    headers = {
        "Content-Type": "application/xml",
    }
    json_data = "<?xml version=\"1.0\"?><item><name>Item</name></item>"
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Unsupported media type"


async def test_xml_response__application_xml(client: Any) -> None:
    """Tests XML response."""
    response = await client.get("/xml")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>"


async def test_14_content_type_case_insensitive(client: Any) -> None:
    """Content-Type header should be case-insensitive."""
    headers = {
        "Content-Type": "APPLICATION/JSON",
    }
    json_data = {"name": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "test"


async def test_json_with_utf_8_charset(client: Any) -> None:
    """Tests JSON response with explicit UTF-8 charset."""
    response = await client.get("/items/unicode")

    assert response.status_code == 200
    response_data = response.json()
    assert "emoji" in response_data
    assert response_data["emoji"] == "☕"
    assert "name" in response_data
    assert response_data["name"] == "Café"


async def test_16_text_plain_not_accepted(client: Any) -> None:
    """text/plain content-type should be rejected when JSON is expected."""
    headers = {
        "Content-Type": "text/plain",
    }
    json_data = "{\"data\": \"value\"}"
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Unsupported Media Type. Expected application/json"


async def test_pdf_response__application_pdf(client: Any) -> None:
    """Tests PDF file response."""
    response = await client.get("/download/document.pdf")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "pdf_binary_data"


async def test_20_content_length_mismatch(client: Any) -> None:
    """Content-Length header mismatch with actual body size should fail."""
    headers = {
        "Content-Type": "application/json",
        "Content-Length": "100",
    }
    json_data = {"value": "short"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Content-Length header does not match actual body size"


async def test_17_vendor_json_accepted(client: Any) -> None:
    """Vendor-specific JSON content-type should be accepted."""
    headers = {
        "Content-Type": "application/vnd.api+json",
    }
    json_data = {"data": "value"}
    response = await client.post("/api/v1/resource", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "value"


async def test_13_json_with_charset_utf16(client: Any) -> None:
    """JSON with UTF-16 charset should be rejected (UTF-8 only)."""
    headers = {
        "Content-Type": "application/json; charset=utf-16",
    }
    json_data = {"value": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."


async def test_json_response__application_json(client: Any) -> None:
    """Tests JSON response with correct Content-Type header."""
    response = await client.get("/items/json")

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0


async def test_15_multipart_boundary_required(client: Any) -> None:
    """Multipart content-type without boundary parameter should fail."""
    headers = {
        "Content-Type": "multipart/form-data",
    }
    response = await client.post("/upload", headers=headers)

    assert response.status_code == 400
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "multipart/form-data requires 'boundary' parameter"


async def test_content_negotiation__accept_header(client: Any) -> None:
    """Tests content negotiation based on Accept header."""
    headers = {
        "Accept": "application/json",
    }
    response = await client.get("/accept-test/1", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == 1
    assert "name" in response_data
    assert response_data["name"] == "Item"


async def test_html_response__text_html(client: Any) -> None:
    """Tests HTML response."""
    response = await client.get("/html")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "<html><body><h1>Hello</h1></body></html>"


async def test_jpeg_image_response__image_jpeg(client: Any) -> None:
    """Tests JPEG image response."""
    response = await client.get("/images/photo.jpg")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "jpeg_binary_data"


async def test_19_missing_content_type_default_json(client: Any) -> None:
    """Missing Content-Type header should default to JSON when body is present."""
    json_data = {"name": "test"}
    response = await client.post("/data", json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "test"


async def test_png_image_response__image_png(client: Any) -> None:
    """Tests PNG image response."""
    response = await client.get("/images/logo.png")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "png_binary_data"


async def test_plain_text_response__text_plain(client: Any) -> None:
    """Tests plain text response."""
    response = await client.get("/text")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "Hello, World!"


async def test_18_content_type_with_multiple_params(client: Any) -> None:
    """Content-Type with multiple parameters should be parsed correctly."""
    headers = {
        "Content-Type": "application/json; charset=utf-8; boundary=something",
    }
    json_data = {"value": "test"}
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == "test"


async def test_csv_response__text_csv(client: Any) -> None:
    """Tests CSV file response."""
    response = await client.get("/export/data.csv")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "id,name,price\n1,Item A,10.0\n2,Item B,20.0"


async def test_binary_response__application_octet_stream(client: Any) -> None:
    """Tests binary data response."""
    response = await client.get("/download/file.bin")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "binary_data_placeholder"



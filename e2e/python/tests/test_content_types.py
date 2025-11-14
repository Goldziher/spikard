"""E2E tests for content_types."""

from spikard.testing import TestClient
from app.main import (
    create_app_content_types_13_json_with_charset_utf16,
    create_app_content_types_14_content_type_case_insensitive,
    create_app_content_types_15_multipart_boundary_required,
    create_app_content_types_16_text_plain_not_accepted,
    create_app_content_types_17_vendor_json_accepted,
    create_app_content_types_18_content_type_with_multiple_params,
    create_app_content_types_19_missing_content_type_default_json,
    create_app_content_types_20_content_length_mismatch,
    create_app_content_types_415_unsupported_media_type,
    create_app_content_types_binary_response_application_octet_stream,
    create_app_content_types_content_negotiation_accept_header,
    create_app_content_types_csv_response_text_csv,
    create_app_content_types_html_response_text_html,
    create_app_content_types_jpeg_image_response_image_jpeg,
    create_app_content_types_json_response_application_json,
    create_app_content_types_json_with_utf_8_charset,
    create_app_content_types_pdf_response_application_pdf,
    create_app_content_types_plain_text_response_text_plain,
    create_app_content_types_png_image_response_image_png,
    create_app_content_types_xml_response_application_xml,
)


async def test_415_unsupported_media_type() -> None:
    """Tests rejection of unsupported content type."""

    app = create_app_content_types_415_unsupported_media_type()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/xml",
    }
    json_data = '<?xml version="1.0"?><item><name>Item</name></item>'
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 415
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Unsupported media type"


async def test_xml_response_application_xml() -> None:
    """Tests XML response."""

    app = create_app_content_types_xml_response_application_xml()
    client = TestClient(app)

    response = await client.get("/xml")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == '<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>'
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/xml"


async def test_14_content_type_case_insensitive() -> None:
    """Content-Type header should be case-insensitive."""

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

    app = create_app_content_types_json_with_utf_8_charset()
    client = TestClient(app)

    response = await client.get("/items/unicode")

    assert response.status_code == 200
    response_data = response.json()
    assert "emoji" in response_data
    assert response_data["emoji"] == "☕"
    assert "name" in response_data
    assert response_data["name"] == "Café"
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/json; charset=utf-8"


async def test_16_text_plain_not_accepted() -> None:
    """text/plain content-type should be rejected when JSON is expected."""

    app = create_app_content_types_16_text_plain_not_accepted()
    client = TestClient(app)

    headers = {
        "Content-Type": "text/plain",
    }
    raw_body = '{"data": "value"}'
    response = await client.post("/data", headers=headers, data=raw_body)

    assert response.status_code == 415
    response_data = response.json()
    assert "error" in response_data
    assert response_data["error"] == "Unsupported Media Type. Expected application/json"


async def test_pdf_response_application_pdf() -> None:
    """Tests PDF file response."""

    app = create_app_content_types_pdf_response_application_pdf()
    client = TestClient(app)

    response = await client.get("/download/document.pdf")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "pdf_binary_data"
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/pdf"
    assert response_headers.get("content-disposition") == "attachment; filename=document.pdf"


async def test_20_content_length_mismatch() -> None:
    """Content-Length header mismatch with actual body size should fail."""

    app = create_app_content_types_20_content_length_mismatch()
    client = TestClient(app)

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


async def test_17_vendor_json_accepted() -> None:
    """Vendor-specific JSON content-type should be accepted."""

    app = create_app_content_types_17_vendor_json_accepted()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/vnd.api+json",
    }
    raw_body = {"data": "value"}
    response = await client.post("/api/v1/resource", headers=headers, data=raw_body)

    assert response.status_code == 201
    response_data = response.json()
    assert "data" in response_data
    assert response_data["data"] == "value"


async def test_13_json_with_charset_utf16() -> None:
    """JSON with UTF-16 charset should be rejected (UTF-8 only)."""

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

    app = create_app_content_types_json_response_application_json()
    client = TestClient(app)

    response = await client.get("/items/json")

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/json"


async def test_15_multipart_boundary_required() -> None:
    """Multipart content-type without boundary parameter should fail."""

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

    app = create_app_content_types_content_negotiation_accept_header()
    client = TestClient(app)

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
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/json"


async def test_html_response_text_html() -> None:
    """Tests HTML response."""

    app = create_app_content_types_html_response_text_html()
    client = TestClient(app)

    response = await client.get("/html")

    assert response.status_code == 200
    assert response.text() == "<html><body><h1>Hello</h1></body></html>"
    response_headers = response.headers
    assert response_headers.get("content-type") == "text/html; charset=utf-8"


async def test_jpeg_image_response_image_jpeg() -> None:
    """Tests JPEG image response."""

    app = create_app_content_types_jpeg_image_response_image_jpeg()
    client = TestClient(app)

    response = await client.get("/images/photo.jpg")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "jpeg_binary_data"
    response_headers = response.headers
    assert response_headers.get("content-type") == "image/jpeg"


async def test_19_missing_content_type_default_json() -> None:
    """Missing Content-Type header should default to JSON when body is present."""

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

    app = create_app_content_types_png_image_response_image_png()
    client = TestClient(app)

    response = await client.get("/images/logo.png")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "png_binary_data"
    response_headers = response.headers
    assert response_headers.get("content-type") == "image/png"


async def test_plain_text_response_text_plain() -> None:
    """Tests plain text response."""

    app = create_app_content_types_plain_text_response_text_plain()
    client = TestClient(app)

    response = await client.get("/text")

    assert response.status_code == 200
    assert response.text() == "Hello, World!"
    response_headers = response.headers
    assert response_headers.get("content-type") == "text/plain; charset=utf-8"


async def test_18_content_type_with_multiple_params() -> None:
    """Content-Type with multiple parameters should be parsed correctly."""

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

    app = create_app_content_types_csv_response_text_csv()
    client = TestClient(app)

    response = await client.get("/export/data.csv")

    assert response.status_code == 200
    assert response.text() == "id,name,price\n1,Item A,10.0\n2,Item B,20.0"
    response_headers = response.headers
    assert response_headers.get("content-type") == "text/csv; charset=utf-8"
    assert response_headers.get("content-disposition") == "attachment; filename=data.csv"


async def test_binary_response_application_octet_stream() -> None:
    """Tests binary data response."""

    app = create_app_content_types_binary_response_application_octet_stream()
    client = TestClient(app)

    response = await client.get("/download/file.bin")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "binary_data_placeholder"
    response_headers = response.headers
    assert response_headers.get("content-type") == "application/octet-stream"
    assert response_headers.get("content-disposition") == "attachment; filename=file.bin"

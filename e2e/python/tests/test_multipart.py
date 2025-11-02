"""E2E tests for multipart."""

import pytest
from typing import Any

async def test_multiple_values_for_same_field_name() -> None:
    """Multiple files uploaded with the same field name (array-like behavior)."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_multiple_values_for_same_field_name

    app = create_app_multipart_multiple_values_for_same_field_name()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_19_file_mime_spoofing_png_as_jpeg() -> None:
    """File with PNG magic number but JPEG MIME type should be rejected (spoofing detection)."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_19_file_mime_spoofing_png_as_jpeg

    app = create_app_multipart_19_file_mime_spoofing_png_as_jpeg()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_20_file_mime_spoofing_jpeg_as_png() -> None:
    """File with JPEG magic number but PNG MIME type should be rejected (spoofing detection)."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_20_file_mime_spoofing_jpeg_as_png

    app = create_app_multipart_20_file_mime_spoofing_jpeg_as_png()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_21_file_pdf_magic_number_success() -> None:
    """File with correct PDF magic number should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_21_file_pdf_magic_number_success

    app = create_app_multipart_21_file_pdf_magic_number_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_content_type_validation_invalid_type() -> None:
    """Tests file upload with disallowed content type."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_content_type_validation_invalid_type

    app = create_app_multipart_content_type_validation_invalid_type()
    client = TestClient(app)

    response = await client.post("/files/images-only")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_pdf_file_upload() -> None:
    """Tests uploading a PDF document."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_pdf_file_upload

    app = create_app_multipart_pdf_file_upload()
    client = TestClient(app)

    response = await client.post("/files/document")

    assert response.status_code == 200
    response_data = response.json()


async def test_file_list_upload_array_of_files() -> None:
    """Tests uploading multiple files as a list parameter."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_file_list_upload_array_of_files

    app = create_app_multipart_file_list_upload_array_of_files()
    client = TestClient(app)

    response = await client.post("/files/list")

    assert response.status_code == 200
    response_data = response.json()


async def test_optional_file_upload_provided() -> None:
    """Tests optional file parameter when file is provided."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_optional_file_upload_provided

    app = create_app_multipart_optional_file_upload_provided()
    client = TestClient(app)

    response = await client.post("/files/optional")

    assert response.status_code == 200
    response_data = response.json()


async def test_file_size_validation_too_large() -> None:
    """Tests file upload exceeding max size limit."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_file_size_validation_too_large

    app = create_app_multipart_file_size_validation_too_large()
    client = TestClient(app)

    response = await client.post("/files/validated")

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "File too large. Maximum size is 1MB"


async def test_mixed_files_and_form_data() -> None:
    """Multipart request with both file uploads and regular form fields."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_mixed_files_and_form_data

    app = create_app_multipart_mixed_files_and_form_data()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_simple_file_upload() -> None:
    """Single file upload with text/plain content type."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_simple_file_upload

    app = create_app_multipart_simple_file_upload()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_empty_file_upload() -> None:
    """Tests uploading a file with zero bytes."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_empty_file_upload

    app = create_app_multipart_empty_file_upload()
    client = TestClient(app)

    response = await client.post("/files/upload")

    assert response.status_code == 200
    response_data = response.json()


async def test_optional_file_upload_missing() -> None:
    """Tests optional file parameter when no file is provided."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_optional_file_upload_missing

    app = create_app_multipart_optional_file_upload_missing()
    client = TestClient(app)

    json_data = {}
    response = await client.post("/files/optional", json=json_data)

    assert response.status_code == 200
    response_data = response.json()


async def test_file_upload_without_filename() -> None:
    """Upload file content without providing a filename."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_file_upload_without_filename

    app = create_app_multipart_file_upload_without_filename()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_18_file_magic_number_jpeg_success() -> None:
    """File with correct JPEG magic number and matching MIME type should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_18_file_magic_number_jpeg_success

    app = create_app_multipart_18_file_magic_number_jpeg_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_22_file_empty_buffer() -> None:
    """File with empty buffer should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_22_file_empty_buffer

    app = create_app_multipart_22_file_empty_buffer()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_17_file_magic_number_png_success() -> None:
    """File with correct PNG magic number and matching MIME type should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_17_file_magic_number_png_success

    app = create_app_multipart_17_file_magic_number_png_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_form_data_without_files() -> None:
    """Multipart form with only text fields, no file uploads."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_form_data_without_files

    app = create_app_multipart_form_data_without_files()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_multiple_file_uploads() -> None:
    """Upload multiple files in a single multipart request."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_multiple_file_uploads

    app = create_app_multipart_multiple_file_uploads()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_file_upload_with_custom_headers() -> None:
    """File upload with additional custom headers in the multipart section."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_file_upload_with_custom_headers

    app = create_app_multipart_file_upload_with_custom_headers()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()


async def test_required_file_upload_missing() -> None:
    """Tests required file parameter when no file is provided."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_required_file_upload_missing

    app = create_app_multipart_required_file_upload_missing()
    client = TestClient(app)

    json_data = {}
    response = await client.post("/files/required", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_image_file_upload() -> None:
    """Tests uploading an image file (JPEG)."""
    from spikard.testing import TestClient
    from app.main import create_app_multipart_image_file_upload

    app = create_app_multipart_image_file_upload()
    client = TestClient(app)

    response = await client.post("/files/image")

    assert response.status_code == 200
    response_data = response.json()



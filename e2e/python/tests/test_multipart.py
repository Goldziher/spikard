"""E2E tests for multipart."""

from spikard.testing import TestClient
from app.main import (
    create_app_multipart_17_file_magic_number_png_success,
    create_app_multipart_18_file_magic_number_jpeg_success,
    create_app_multipart_19_file_mime_spoofing_png_as_jpeg,
    create_app_multipart_20_file_mime_spoofing_jpeg_as_png,
    create_app_multipart_21_file_pdf_magic_number_success,
    create_app_multipart_22_file_empty_buffer,
    create_app_multipart_content_type_validation_invalid_type,
    create_app_multipart_empty_file_upload,
    create_app_multipart_file_list_upload_array_of_files,
    create_app_multipart_file_size_validation_too_large,
    create_app_multipart_file_upload_with_custom_headers,
    create_app_multipart_file_upload_without_filename,
    create_app_multipart_form_data_without_files,
    create_app_multipart_image_file_upload,
    create_app_multipart_mixed_files_and_form_data,
    create_app_multipart_multiple_file_uploads,
    create_app_multipart_multiple_values_for_same_field_name,
    create_app_multipart_optional_file_upload_missing,
    create_app_multipart_optional_file_upload_provided,
    create_app_multipart_pdf_file_upload,
    create_app_multipart_required_file_upload_missing,
    create_app_multipart_simple_file_upload,
)


async def test_multiple_values_for_same_field_name() -> None:
    """Multiple files uploaded with the same field name (array-like behavior)."""

    async with TestClient(create_app_multipart_multiple_values_for_same_field_name()) as client:
        data = {"tags": ["python", "rust", "web"]}
        files = [
            ("files", ("file1.txt", b"first file", "text/plain")),
            ("files", ("file2.txt", b"second file", "text/plain")),
        ]
        response = await client.post("/", data=data, files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "files" in response_data
        assert len(response_data["files"]) == 2
        assert "filename" in response_data["files"][0]
        assert response_data["files"][0]["filename"] == "file1.txt"
        assert "size" in response_data["files"][0]
        assert response_data["files"][0]["size"] == 10
        assert "content" in response_data["files"][0]
        assert response_data["files"][0]["content"] == "first file"
        assert "content_type" in response_data["files"][0]
        assert response_data["files"][0]["content_type"] == "text/plain"
        assert "filename" in response_data["files"][1]
        assert response_data["files"][1]["filename"] == "file2.txt"
        assert "size" in response_data["files"][1]
        assert response_data["files"][1]["size"] == 11
        assert "content" in response_data["files"][1]
        assert response_data["files"][1]["content"] == "second file"
        assert "content_type" in response_data["files"][1]
        assert response_data["files"][1]["content_type"] == "text/plain"
        assert "tags" in response_data
        assert len(response_data["tags"]) == 3
        assert response_data["tags"][0] == "python"
        assert response_data["tags"][1] == "rust"
        assert response_data["tags"][2] == "web"


async def test_19_file_mime_spoofing_png_as_jpeg() -> None:
    """File with PNG magic number but JPEG MIME type should be rejected (spoofing detection)."""

    async with TestClient(create_app_multipart_19_file_mime_spoofing_png_as_jpeg()) as client:
        files = {
            "image": ("fake.jpg", bytes.fromhex("89504e470d0a1a0a"), "image/jpeg"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_20_file_mime_spoofing_jpeg_as_png() -> None:
    """File with JPEG magic number but PNG MIME type should be rejected (spoofing detection)."""

    async with TestClient(create_app_multipart_20_file_mime_spoofing_jpeg_as_png()) as client:
        files = {
            "image": ("fake.png", bytes.fromhex("ffd8ffe0"), "image/png"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_21_file_pdf_magic_number_success() -> None:
    """File with correct PDF magic number should be accepted."""

    async with TestClient(create_app_multipart_21_file_pdf_magic_number_success()) as client:
        files = {
            "document": ("test.pdf", bytes.fromhex("25504446"), "application/pdf"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 201


async def test_content_type_validation_invalid_type() -> None:
    """Tests file upload with disallowed content type."""

    async with TestClient(create_app_multipart_content_type_validation_invalid_type()) as client:
        files = {
            "file": ("script.sh", b"#!/bin/bash\necho hello", "application/x-sh"),
        }
        response = await client.post("/files/images-only", files=files)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data
        response_data = response.json()
        assert "errors" in response_data
        assert len(response_data["errors"]) == 1
        error_0 = response_data["errors"][0]
        assert error_0["type"] == "validation_error"
        assert error_0["loc"] == ["files", "file"]
        assert (
            error_0["msg"] == "Invalid content type 'application/x-sh'. Allowed types: image/jpeg, image/png, image/gif"
        )


async def test_pdf_file_upload() -> None:
    """Tests uploading a PDF document."""

    async with TestClient(create_app_multipart_pdf_file_upload()) as client:
        files = {
            "document": ("report.pdf", b"fake_pdf_content", "application/pdf"),
        }
        response = await client.post("/files/document", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "filename" in response_data
        assert response_data["filename"] == "report.pdf"
        assert "content_type" in response_data
        assert response_data["content_type"] == "application/pdf"
        assert "size" in response_data
        assert response_data["size"] == 16


async def test_file_list_upload_array_of_files() -> None:
    """Tests uploading multiple files as a list parameter."""

    async with TestClient(create_app_multipart_file_list_upload_array_of_files()) as client:
        files = [
            ("files", ("file1.txt", b"content of file 1", "text/plain")),
            ("files", ("file2.txt", b"content of file 2", "text/plain")),
        ]
        response = await client.post("/files/list", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "filenames" in response_data
        assert len(response_data["filenames"]) == 2
        assert response_data["filenames"][0] == "file1.txt"
        assert response_data["filenames"][1] == "file2.txt"
        assert "total_size" in response_data
        assert response_data["total_size"] == 35


async def test_optional_file_upload_provided() -> None:
    """Tests optional file parameter when file is provided."""

    async with TestClient(create_app_multipart_optional_file_upload_provided()) as client:
        files = {
            "file": ("optional.txt", b"optional file content", "text/plain"),
        }
        response = await client.post("/files/optional", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "filename" in response_data
        assert response_data["filename"] == "optional.txt"
        assert "content_type" in response_data
        assert response_data["content_type"] == "text/plain"
        assert "size" in response_data
        assert response_data["size"] == 21


async def test_file_size_validation_too_large() -> None:
    """Tests file upload exceeding max size limit."""

    async with TestClient(create_app_multipart_file_size_validation_too_large()) as client:
        files = {
            "file": ("large.txt", b"x", "text/plain"),
        }
        response = await client.post("/files/validated", files=files)

        assert response.status_code == 413
        response_data = response.json()
        assert "detail" in response_data
        assert response_data["detail"] == "File too large. Maximum size is 1MB"


async def test_mixed_files_and_form_data() -> None:
    """Multipart request with both file uploads and regular form fields."""

    async with TestClient(create_app_multipart_mixed_files_and_form_data()) as client:
        data = {"active": "true", "age": "25", "username": "testuser"}
        files = {
            "file": ("upload.txt", b"file data here", "text/plain"),
        }
        response = await client.post("/", data=data, files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "file" in response_data
        assert "filename" in response_data["file"]
        assert response_data["file"]["filename"] == "upload.txt"
        assert "size" in response_data["file"]
        assert response_data["file"]["size"] == 14
        assert "content" in response_data["file"]
        assert response_data["file"]["content"] == "file data here"
        assert "content_type" in response_data["file"]
        assert response_data["file"]["content_type"] == "text/plain"
        assert "username" in response_data
        assert response_data["username"] == "testuser"
        assert "age" in response_data
        assert response_data["age"] == "25"
        assert "active" in response_data
        assert response_data["active"] == "true"


async def test_simple_file_upload() -> None:
    """Single file upload with text/plain content type."""

    async with TestClient(create_app_multipart_simple_file_upload()) as client:
        files = {
            "test": ("test.txt", b"<file content>", "text/plain"),
        }
        response = await client.post("/", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "test" in response_data
        assert "filename" in response_data["test"]
        assert response_data["test"]["filename"] == "test.txt"
        assert "size" in response_data["test"]
        assert response_data["test"]["size"] == 14
        assert "content" in response_data["test"]
        assert response_data["test"]["content"] == "<file content>"
        assert "content_type" in response_data["test"]
        assert response_data["test"]["content_type"] == "text/plain"


async def test_empty_file_upload() -> None:
    """Tests uploading a file with zero bytes."""

    async with TestClient(create_app_multipart_empty_file_upload()) as client:
        files = {
            "file": ("empty.txt", b"", "text/plain"),
        }
        response = await client.post("/files/upload", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "filename" in response_data
        assert response_data["filename"] == "empty.txt"
        assert "size" in response_data
        assert response_data["size"] == 0


async def test_optional_file_upload_missing() -> None:
    """Tests optional file parameter when no file is provided."""

    async with TestClient(create_app_multipart_optional_file_upload_missing()) as client:
        response = await client.post("/files/optional")

        assert response.status_code == 200
        response_data = response.json()
        assert "file" in response_data
        assert response_data["file"] == None


async def test_file_upload_without_filename() -> None:
    """Upload file content without providing a filename."""

    async with TestClient(create_app_multipart_file_upload_without_filename()) as client:
        files = {
            "test1": ("file.txt", b"<file1 content>"),
        }
        response = await client.post("/", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "test1" in response_data
        assert response_data["test1"] == "<file1 content>"


async def test_18_file_magic_number_jpeg_success() -> None:
    """File with correct JPEG magic number and matching MIME type should be accepted."""

    async with TestClient(create_app_multipart_18_file_magic_number_jpeg_success()) as client:
        files = {
            "image": ("test.jpg", bytes.fromhex("ffd8ffe0"), "image/jpeg"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 201


async def test_22_file_empty_buffer() -> None:
    """File with empty buffer should fail validation."""

    async with TestClient(create_app_multipart_22_file_empty_buffer()) as client:
        files = {
            "file": ("empty.txt", bytes.fromhex(""), "text/plain"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_17_file_magic_number_png_success() -> None:
    """File with correct PNG magic number and matching MIME type should be accepted."""

    async with TestClient(create_app_multipart_17_file_magic_number_png_success()) as client:
        files = {
            "image": ("test.png", bytes.fromhex("89504e470d0a1a0a"), "image/png"),
        }
        response = await client.post("/upload", files=files)

        assert response.status_code == 201


async def test_form_data_without_files() -> None:
    """Multipart form with only text fields, no file uploads."""

    async with TestClient(create_app_multipart_form_data_without_files()) as client:
        data = {"some": "data"}
        response = await client.post("/", data=data)

        assert response.status_code == 200
        response_data = response.json()
        assert "some" in response_data
        assert response_data["some"] == "data"


async def test_multiple_file_uploads() -> None:
    """Upload multiple files in a single multipart request."""

    async with TestClient(create_app_multipart_multiple_file_uploads()) as client:
        files = {
            "test1": ("test1.txt", b"<file1 content>", "text/plain"),
            "test2": ("test2.txt", b"<file2 content>", "text/plain"),
        }
        response = await client.post("/", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "test1" in response_data
        assert "filename" in response_data["test1"]
        assert response_data["test1"]["filename"] == "test1.txt"
        assert "size" in response_data["test1"]
        assert response_data["test1"]["size"] == 15
        assert "content" in response_data["test1"]
        assert response_data["test1"]["content"] == "<file1 content>"
        assert "content_type" in response_data["test1"]
        assert response_data["test1"]["content_type"] == "text/plain"
        assert "test2" in response_data
        assert "filename" in response_data["test2"]
        assert response_data["test2"]["filename"] == "test2.txt"
        assert "size" in response_data["test2"]
        assert response_data["test2"]["size"] == 15
        assert "content" in response_data["test2"]
        assert response_data["test2"]["content"] == "<file2 content>"
        assert "content_type" in response_data["test2"]
        assert response_data["test2"]["content_type"] == "text/plain"


async def test_file_upload_with_custom_headers() -> None:
    """File upload with additional custom headers in the multipart section."""

    async with TestClient(create_app_multipart_file_upload_with_custom_headers()) as client:
        files = {
            "test2": ("test2.txt", b"<file2 content>", "text/plain"),
        }
        response = await client.post("/", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "test2" in response_data
        assert "filename" in response_data["test2"]
        assert response_data["test2"]["filename"] == "test2.txt"
        assert "size" in response_data["test2"]
        assert response_data["test2"]["size"] == 15
        assert "content" in response_data["test2"]
        assert response_data["test2"]["content"] == "<file2 content>"
        assert "content_type" in response_data["test2"]
        assert response_data["test2"]["content_type"] == "text/plain"
        assert "headers" in response_data["test2"]
        assert len(response_data["test2"]["headers"]) == 3
        assert len(response_data["test2"]["headers"][0]) == 2
        assert response_data["test2"]["headers"][0][0] == "content-disposition"
        assert response_data["test2"]["headers"][0][1] == 'form-data; name="test2"; filename="test2.txt"'
        assert len(response_data["test2"]["headers"][1]) == 2
        assert response_data["test2"]["headers"][1][0] == "content-type"
        assert response_data["test2"]["headers"][1][1] == "text/plain"
        assert len(response_data["test2"]["headers"][2]) == 2
        assert response_data["test2"]["headers"][2][0] == "x-custom"
        assert response_data["test2"]["headers"][2][1] == "f2"


async def test_required_file_upload_missing() -> None:
    """Tests required file parameter when no file is provided."""

    async with TestClient(create_app_multipart_required_file_upload_missing()) as client:
        response = await client.post("/files/required")

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_image_file_upload() -> None:
    """Tests uploading an image file (JPEG)."""

    async with TestClient(create_app_multipart_image_file_upload()) as client:
        files = {
            "image": ("photo.jpg", b"fake_jpeg_content_here", "image/jpeg"),
        }
        response = await client.post("/files/image", files=files)

        assert response.status_code == 200
        response_data = response.json()
        assert "filename" in response_data
        assert response_data["filename"] == "photo.jpg"
        assert "content_type" in response_data
        assert response_data["content_type"] == "image/jpeg"
        assert "size" in response_data
        assert response_data["size"] == 22

"""E2E tests for multipart."""


async def test_multiple_values_for_same_field_name() -> None:
    """Multiple files uploaded with the same field name (array-like behavior)."""
    from app.main import create_app_multipart_multiple_values_for_same_field_name

    from spikard.testing import TestClient

    app = create_app_multipart_multiple_values_for_same_field_name()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "files" in response_data
    assert len(response_data["files"]) == 2
    assert "content" in response_data["files"][0]
    assert response_data["files"][0]["content"] == "first file"
    assert "content_type" in response_data["files"][0]
    assert response_data["files"][0]["content_type"] == "text/plain"
    assert "filename" in response_data["files"][0]
    assert response_data["files"][0]["filename"] == "file1.txt"
    assert "size" in response_data["files"][0]
    assert response_data["files"][0]["size"] == 10
    assert "content" in response_data["files"][1]
    assert response_data["files"][1]["content"] == "second file"
    assert "content_type" in response_data["files"][1]
    assert response_data["files"][1]["content_type"] == "text/plain"
    assert "filename" in response_data["files"][1]
    assert response_data["files"][1]["filename"] == "file2.txt"
    assert "size" in response_data["files"][1]
    assert response_data["files"][1]["size"] == 11
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "python"
    assert response_data["tags"][1] == "rust"
    assert response_data["tags"][2] == "web"


async def test_19_file_mime_spoofing_png_as_jpeg() -> None:
    """File with PNG magic number but JPEG MIME type should be rejected (spoofing detection)."""
    from app.main import create_app_multipart_19_file_mime_spoofing_png_as_jpeg

    from spikard.testing import TestClient

    app = create_app_multipart_19_file_mime_spoofing_png_as_jpeg()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "declared_mime" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["declared_mime"] == "image/jpeg"
    assert "detected_type" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["detected_type"] == "image/png"
    assert "magic_bytes" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["magic_bytes"] == "89504e470d0a1a0a"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "files"
    assert response_data["errors"][0]["loc"][1] == "image"
    assert "msg" in response_data["errors"][0]
    assert (
        response_data["errors"][0]["msg"]
        == "File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png"
    )
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_20_file_mime_spoofing_jpeg_as_png() -> None:
    """File with JPEG magic number but PNG MIME type should be rejected (spoofing detection)."""
    from app.main import create_app_multipart_20_file_mime_spoofing_jpeg_as_png

    from spikard.testing import TestClient

    app = create_app_multipart_20_file_mime_spoofing_jpeg_as_png()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "declared_mime" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["declared_mime"] == "image/png"
    assert "detected_type" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["detected_type"] == "image/jpeg"
    assert "magic_bytes" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["magic_bytes"] == "ffd8ffe0"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "files"
    assert response_data["errors"][0]["loc"][1] == "image"
    assert "msg" in response_data["errors"][0]
    assert (
        response_data["errors"][0]["msg"]
        == "File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg"
    )
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_21_file_pdf_magic_number_success() -> None:
    """File with correct PDF magic number should be accepted."""
    from app.main import create_app_multipart_21_file_pdf_magic_number_success

    from spikard.testing import TestClient

    app = create_app_multipart_21_file_pdf_magic_number_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_content_type_validation_invalid_type() -> None:
    """Tests file upload with disallowed content type."""
    from app.main import create_app_multipart_content_type_validation_invalid_type

    from spikard.testing import TestClient

    app = create_app_multipart_content_type_validation_invalid_type()
    client = TestClient(app)

    response = await client.post("/files/images-only")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Only image files are allowed (image/jpeg, image/png, image/gif)"


async def test_pdf_file_upload() -> None:
    """Tests uploading a PDF document."""
    from app.main import create_app_multipart_pdf_file_upload

    from spikard.testing import TestClient

    app = create_app_multipart_pdf_file_upload()
    client = TestClient(app)

    response = await client.post("/files/document")

    assert response.status_code == 200
    response_data = response.json()
    assert "content_type" in response_data
    assert response_data["content_type"] == "application/pdf"
    assert "filename" in response_data
    assert response_data["filename"] == "report.pdf"
    assert "size" in response_data
    assert response_data["size"] == 16


async def test_file_list_upload_array_of_files() -> None:
    """Tests uploading multiple files as a list parameter."""
    from app.main import create_app_multipart_file_list_upload_array_of_files

    from spikard.testing import TestClient

    app = create_app_multipart_file_list_upload_array_of_files()
    client = TestClient(app)

    response = await client.post("/files/list")

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
    from app.main import create_app_multipart_optional_file_upload_provided

    from spikard.testing import TestClient

    app = create_app_multipart_optional_file_upload_provided()
    client = TestClient(app)

    response = await client.post("/files/optional")

    assert response.status_code == 200
    response_data = response.json()
    assert "content_type" in response_data
    assert response_data["content_type"] == "text/plain"
    assert "filename" in response_data
    assert response_data["filename"] == "optional.txt"
    assert "size" in response_data
    assert response_data["size"] == 21


async def test_file_size_validation_too_large() -> None:
    """Tests file upload exceeding max size limit."""
    from app.main import create_app_multipart_file_size_validation_too_large

    from spikard.testing import TestClient

    app = create_app_multipart_file_size_validation_too_large()
    client = TestClient(app)

    response = await client.post("/files/validated")

    assert response.status_code == 413
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "File too large. Maximum size is 1MB"


async def test_mixed_files_and_form_data() -> None:
    """Multipart request with both file uploads and regular form fields."""
    from app.main import create_app_multipart_mixed_files_and_form_data

    from spikard.testing import TestClient

    app = create_app_multipart_mixed_files_and_form_data()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "active" in response_data
    assert response_data["active"] == "true"
    assert "age" in response_data
    assert response_data["age"] == "25"
    assert "file" in response_data
    assert "content" in response_data["file"]
    assert response_data["file"]["content"] == "file data here"
    assert "content_type" in response_data["file"]
    assert response_data["file"]["content_type"] == "text/plain"
    assert "filename" in response_data["file"]
    assert response_data["file"]["filename"] == "upload.txt"
    assert "size" in response_data["file"]
    assert response_data["file"]["size"] == 14
    assert "username" in response_data
    assert response_data["username"] == "testuser"


async def test_simple_file_upload() -> None:
    """Single file upload with text/plain content type."""
    from app.main import create_app_multipart_simple_file_upload

    from spikard.testing import TestClient

    app = create_app_multipart_simple_file_upload()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "test" in response_data
    assert "content" in response_data["test"]
    assert response_data["test"]["content"] == "<file content>"
    assert "content_type" in response_data["test"]
    assert response_data["test"]["content_type"] == "text/plain"
    assert "filename" in response_data["test"]
    assert response_data["test"]["filename"] == "test.txt"
    assert "size" in response_data["test"]
    assert response_data["test"]["size"] == 14


async def test_empty_file_upload() -> None:
    """Tests uploading a file with zero bytes."""
    from app.main import create_app_multipart_empty_file_upload

    from spikard.testing import TestClient

    app = create_app_multipart_empty_file_upload()
    client = TestClient(app)

    response = await client.post("/files/upload")

    assert response.status_code == 200
    response_data = response.json()
    assert "filename" in response_data
    assert response_data["filename"] == "empty.txt"
    assert "size" in response_data
    assert response_data["size"] == 0


async def test_optional_file_upload_missing() -> None:
    """Tests optional file parameter when no file is provided."""
    from app.main import create_app_multipart_optional_file_upload_missing

    from spikard.testing import TestClient

    app = create_app_multipart_optional_file_upload_missing()
    client = TestClient(app)

    json_data = {}
    response = await client.post("/files/optional", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "file" in response_data
    assert response_data["file"] is None


async def test_file_upload_without_filename() -> None:
    """Upload file content without providing a filename."""
    from app.main import create_app_multipart_file_upload_without_filename

    from spikard.testing import TestClient

    app = create_app_multipart_file_upload_without_filename()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "test1" in response_data
    assert response_data["test1"] == "<file1 content>"


async def test_18_file_magic_number_jpeg_success() -> None:
    """File with correct JPEG magic number and matching MIME type should be accepted."""
    from app.main import create_app_multipart_18_file_magic_number_jpeg_success

    from spikard.testing import TestClient

    app = create_app_multipart_18_file_magic_number_jpeg_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_22_file_empty_buffer() -> None:
    """File with empty buffer should fail validation."""
    from app.main import create_app_multipart_22_file_empty_buffer

    from spikard.testing import TestClient

    app = create_app_multipart_22_file_empty_buffer()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "buffer_size" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["buffer_size"] == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "files"
    assert response_data["errors"][0]["loc"][1] == "file"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "File buffer is empty"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_17_file_magic_number_png_success() -> None:
    """File with correct PNG magic number and matching MIME type should be accepted."""
    from app.main import create_app_multipart_17_file_magic_number_png_success

    from spikard.testing import TestClient

    app = create_app_multipart_17_file_magic_number_png_success()
    client = TestClient(app)

    response = await client.post("/upload")

    assert response.status_code == 201


async def test_form_data_without_files() -> None:
    """Multipart form with only text fields, no file uploads."""
    from app.main import create_app_multipart_form_data_without_files

    from spikard.testing import TestClient

    app = create_app_multipart_form_data_without_files()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "some" in response_data
    assert response_data["some"] == "data"


async def test_multiple_file_uploads() -> None:
    """Upload multiple files in a single multipart request."""
    from app.main import create_app_multipart_multiple_file_uploads

    from spikard.testing import TestClient

    app = create_app_multipart_multiple_file_uploads()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "test1" in response_data
    assert "content" in response_data["test1"]
    assert response_data["test1"]["content"] == "<file1 content>"
    assert "content_type" in response_data["test1"]
    assert response_data["test1"]["content_type"] == "text/plain"
    assert "filename" in response_data["test1"]
    assert response_data["test1"]["filename"] == "test1.txt"
    assert "size" in response_data["test1"]
    assert response_data["test1"]["size"] == 15
    assert "test2" in response_data
    assert "content" in response_data["test2"]
    assert response_data["test2"]["content"] == "<file2 content>"
    assert "content_type" in response_data["test2"]
    assert response_data["test2"]["content_type"] == "text/plain"
    assert "filename" in response_data["test2"]
    assert response_data["test2"]["filename"] == "test2.txt"
    assert "size" in response_data["test2"]
    assert response_data["test2"]["size"] == 15


async def test_file_upload_with_custom_headers() -> None:
    """File upload with additional custom headers in the multipart section."""
    from app.main import create_app_multipart_file_upload_with_custom_headers

    from spikard.testing import TestClient

    app = create_app_multipart_file_upload_with_custom_headers()
    client = TestClient(app)

    response = await client.post("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "test2" in response_data
    assert "content" in response_data["test2"]
    assert response_data["test2"]["content"] == "<file2 content>"
    assert "content_type" in response_data["test2"]
    assert response_data["test2"]["content_type"] == "text/plain"
    assert "filename" in response_data["test2"]
    assert response_data["test2"]["filename"] == "test2.txt"
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
    assert "size" in response_data["test2"]
    assert response_data["test2"]["size"] == 15


async def test_required_file_upload_missing() -> None:
    """Tests required file parameter when no file is provided."""
    from app.main import create_app_multipart_required_file_upload_missing

    from spikard.testing import TestClient

    app = create_app_multipart_required_file_upload_missing()
    client = TestClient(app)

    json_data = {}
    response = await client.post("/files/required", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "required"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "file"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_image_file_upload() -> None:
    """Tests uploading an image file (JPEG)."""
    from app.main import create_app_multipart_image_file_upload

    from spikard.testing import TestClient

    app = create_app_multipart_image_file_upload()
    client = TestClient(app)

    response = await client.post("/files/image")

    assert response.status_code == 200
    response_data = response.json()
    assert "content_type" in response_data
    assert response_data["content_type"] == "image/jpeg"
    assert "filename" in response_data
    assert response_data["filename"] == "photo.jpg"
    assert "size" in response_data
    assert response_data["size"] == 22

"""Tests for UploadFile datastructure and multipart handling."""

from dataclasses import dataclass

import pytest

from spikard import Spikard, UploadFile
from spikard.testing import TestClient


class TestUploadFileAPI:
    """Test UploadFile class API (sync)."""

    def test_basic_properties(self) -> None:
        """Test basic UploadFile properties."""
        content = b"Hello, World!"
        upload = UploadFile(
            filename="test.txt",
            content=content,
            content_type="text/plain",
        )

        assert upload.filename == "test.txt"
        assert upload.content_type == "text/plain"
        assert upload.size == len(content)
        assert repr(upload) == "UploadFile(filename='test.txt', content_type='text/plain', size=13)"

    def test_read(self) -> None:
        """Test reading file content."""
        content = b"Hello, World!"
        upload = UploadFile(filename="test.txt", content=content)

        assert upload.read() == content
        assert upload.read() == b""

        upload.seek(0)
        assert upload.read() == content

    def test_partial_read(self) -> None:
        """Test partial reads."""
        content = b"Hello, World!"
        upload = UploadFile(filename="test.txt", content=content)

        assert upload.read(5) == b"Hello"
        assert upload.read(2) == b", "
        assert upload.read(6) == b"World!"

    def test_write(self) -> None:
        """Test writing to file."""
        upload = UploadFile(filename="test.txt", content=b"Initial")

        upload.seek(0, 2)
        bytes_written = upload.write(b" content")
        assert bytes_written == 8

        upload.seek(0)
        assert upload.read() == b"Initial content"

    def test_context_manager(self) -> None:
        """Test context manager support."""
        content = b"Hello, World!"
        upload = UploadFile(filename="test.txt", content=content)

        with upload as f:
            data = f.read()
            assert data == content

    async def test_async_read(self) -> None:
        """Test async read."""
        content = b"Hello, World!"
        upload = UploadFile(filename="test.txt", content=content)

        assert await upload.aread() == content

    async def test_async_context_manager(self) -> None:
        """Test async context manager."""
        content = b"Hello, World!"
        upload = UploadFile(filename="test.txt", content=content)

        async with upload as f:
            data = await f.aread()
            assert data == content


@pytest.mark.asyncio
class TestMultipartUpload:
    """Test multipart file uploads through HTTP."""

    async def test_single_file_upload(self) -> None:
        """Test uploading a single file."""

        @dataclass
        class UploadRequest:
            file: UploadFile
            description: str

        app = Spikard()

        @app.post("/upload")
        def upload_handler(body: UploadRequest) -> dict[str, str | int]:
            return {
                "filename": body.file.filename,
                "size": body.file.size,
                "content_type": body.file.content_type,
                "description": body.description,
                "content": body.file.read().decode("utf-8"),
            }

        async with TestClient(app) as client:
            response = await client.post(
                "/upload",
                files={"file": ("test.txt", b"Hello, World!", "text/plain")},
                data={"description": "Test upload"},
            )

            if response.status_code != 200:
                pass
            assert response.status_code == 200
            data = response.json()
            assert data["filename"] == "test.txt"
            assert data["content_type"] == "text/plain"
            assert data["description"] == "Test upload"
            assert data["content"] == "Hello, World!"

    async def test_multiple_files_upload(self) -> None:
        """Test uploading multiple files."""
        app = Spikard()

        @app.post("/upload-many")
        def upload_multiple(files: list[UploadFile]) -> dict[str, int | list[str | int]]:
            return {
                "count": len(files),
                "filenames": [f.filename for f in files],
                "sizes": [f.size for f in files],
            }

        async with TestClient(app) as client:
            response = await client.post(
                "/upload-many",
                files=[
                    ("files", ("file1.txt", b"Content 1")),
                    ("files", ("file2.txt", b"Content 2")),
                ],
            )

            if response.status_code != 200:
                pass
            assert response.status_code == 200
            data = response.json()
            assert data["count"] == 2
            assert data["filenames"] == ["file1.txt", "file2.txt"]

    async def test_optional_file_upload(self) -> None:
        """Test optional file upload."""

        @dataclass
        class OptionalUpload:
            file: UploadFile | None
            name: str

        app = Spikard()

        @app.post("/upload-optional")
        def upload_optional(body: OptionalUpload) -> dict[str, str | bool | None]:
            return {
                "has_file": body.file is not None,
                "name": body.name,
                "filename": body.file.filename if body.file else None,
            }

        async with TestClient(app) as client:
            response = await client.post(
                "/upload-optional",
                files={"file": ("test.txt", b"data")},
                data={"name": "Test"},
            )
            assert response.status_code == 200
            data = response.json()
            assert data["has_file"] is True
            assert data["filename"] == "test.txt"

            response = await client.post("/upload-optional", data={"name": "Test"})
            if response.status_code != 200:
                pass
            assert response.status_code == 200
            data = response.json()
            assert data["has_file"] is False
            assert data["filename"] is None

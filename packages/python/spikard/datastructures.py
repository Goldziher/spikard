"""Data structures for HTTP handling.

This module provides data structures for handling file uploads and other
HTTP-related data. The UploadFile API is designed to be compatible with
Litestar and FastAPI patterns while optimized for Spikard's Rust-backed
request processing.
"""

from __future__ import annotations

import io
from tempfile import SpooledTemporaryFile
from typing import Annotated, Any

try:
    from typing import Self
except ImportError:  # pragma: no cover - py310 fallback
    from typing_extensions import Self  # noqa: UP035

import msgspec

__all__ = ("UploadFile",)


_UPLOAD_FILE_JSON_SCHEMA = {
    "type": "object",
    "properties": {
        "filename": {"type": "string"},
        "content": {"type": "string", "format": "binary"},
        "size": {"type": "integer"},
        "content_type": {"type": "string"},
    },
    "required": ["filename", "content"],
    "format": "binary",
    "description": "File upload",
}


UploadFileType = Annotated[Any, msgspec.Meta(extra_json_schema=_UPLOAD_FILE_JSON_SCHEMA)]


class UploadFile:
    """Represents an uploaded file from multipart/form-data requests.

    This class provides both sync and async interfaces for file operations,
    with automatic spooling to disk for large files. The API is compatible
    with Litestar and FastAPI's UploadFile interface.

    Example:
        ```python
        from dataclasses import dataclass
        from spikard import Spikard
        from spikard.datastructures import UploadFile

        app = Spikard()


        @dataclass
        class UploadRequest:
            file: UploadFile
            description: str


        @app.post("/upload")
        async def upload_file(body: UploadRequest):
            content = await body.file.read()
            return {
                "filename": body.file.filename,
                "size": body.file.size,
                "content_type": body.file.content_type,
                "description": body.description,
            }
        ```

    Attributes:
        filename: The original filename from the client
        content_type: MIME type of the uploaded file
        size: Size of the file in bytes
        headers: Additional headers associated with this file field
    """

    __slots__ = ("_content", "_file", "content_type", "filename", "headers", "size")

    def __init__(
        self,
        filename: str,
        content: bytes,
        content_type: str | None = None,
        size: int | None = None,
        headers: dict[str, str] | None = None,
        max_spool_size: int = 1024 * 1024,
    ) -> None:
        """Initialize an UploadFile instance.

        Args:
            filename: The original filename
            content: Raw file bytes
            content_type: MIME type (defaults to "application/octet-stream")
            size: File size in bytes (computed from content if not provided)
            headers: Additional headers from the multipart field
            max_spool_size: Size threshold for spooling to disk (default 1MB)
        """
        self.filename = filename
        self.content_type = content_type or "application/octet-stream"
        self.size = size if size is not None else len(content)
        self.headers = headers or {}
        self._content = content

        self._file: SpooledTemporaryFile[bytes] = SpooledTemporaryFile(max_size=max_spool_size, mode="w+b")  # noqa: SIM115
        self._file.write(content)
        self._file.seek(0)

    @property
    def rolled_to_disk(self) -> bool:
        """Check if the file exceeded the memory threshold and was spooled to disk.

        Returns:
            True if the file is on disk, False if still in memory
        """
        return getattr(self._file, "_rolled", False)

    def read(self, size: int = -1) -> bytes:
        """Read file contents synchronously.

        Args:
            size: Number of bytes to read (-1 for all)

        Returns:
            File contents as bytes
        """
        return self._file.read(size)

    async def aread(self, size: int = -1) -> bytes:
        """Read file contents asynchronously.

        For files in memory, this is a simple wrapper. For files spooled to disk,
        this would ideally use anyio/trio file I/O, but for now returns synchronously.

        Args:
            size: Number of bytes to read (-1 for all)

        Returns:
            File contents as bytes
        """
        # TODO: For true async file I/O when rolled_to_disk, integrate with anyio/trio
        return self.read(size)

    def seek(self, offset: int, whence: int = 0) -> int:
        """Seek to a position in the file synchronously.

        Args:
            offset: Position to seek to
            whence: How to interpret offset (0=absolute, 1=relative, 2=from end)

        Returns:
            New absolute position
        """
        return self._file.seek(offset, whence)

    async def aseek(self, offset: int, whence: int = 0) -> int:
        """Seek to a position in the file asynchronously.

        Args:
            offset: Position to seek to
            whence: How to interpret offset (0=absolute, 1=relative, 2=from end)

        Returns:
            New absolute position
        """
        # TODO: Async version for disk-spooled files
        return self.seek(offset, whence)

    def write(self, data: bytes) -> int:
        """Write data to the file synchronously.

        Args:
            data: Bytes to write

        Returns:
            Number of bytes written
        """
        current_pos = self._file.tell()
        bytes_written = self._file.write(data)
        end_pos = self._file.tell()
        self.size = max(self.size, end_pos)
        self._file.seek(current_pos + bytes_written)
        return bytes_written

    async def awrite(self, data: bytes) -> int:
        """Write data to the file asynchronously.

        Args:
            data: Bytes to write

        Returns:
            Number of bytes written
        """
        # TODO: Async version for disk-spooled files
        return self.write(data)

    def close(self) -> None:
        """Close the file synchronously."""
        if not self._file.closed:
            self._file.close()

    async def aclose(self) -> None:
        """Close the file asynchronously."""
        # TODO: Async version for disk-spooled files
        self.close()

    @property
    def file(self) -> SpooledTemporaryFile[bytes]:
        """Get the underlying SpooledTemporaryFile for direct access.

        Returns:
            The underlying file object
        """
        return self._file

    def as_bytes_io(self) -> io.BytesIO:
        """Get file contents as a BytesIO object.

        This creates a new BytesIO with the current file contents.
        Useful for libraries that expect a file-like object.

        Returns:
            BytesIO containing file contents
        """
        current_pos = self._file.tell()
        self._file.seek(0)
        data = self._file.read()
        self._file.seek(current_pos)
        return io.BytesIO(data)

    def __repr__(self) -> str:
        """String representation of the upload file."""
        return f"UploadFile(filename={self.filename!r}, content_type={self.content_type!r}, size={self.size})"

    def __del__(self) -> None:
        """Ensure file is closed when object is garbage collected."""
        if hasattr(self, "_file") and not self._file.closed:
            self._file.close()

    def __enter__(self) -> Self:
        """Enter context manager (sync)."""
        return self

    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: object) -> None:
        """Exit context manager (sync)."""
        self.close()

    async def __aenter__(self) -> Self:
        """Enter async context manager."""
        return self

    async def __aexit__(
        self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: object
    ) -> None:
        """Exit async context manager."""
        await self.aclose()

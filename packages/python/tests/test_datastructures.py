"""Advanced tests for UploadFile datastructure edge cases.

This module provides comprehensive coverage for:
- Line 119: rolled_to_disk property checking SpooledTemporaryFile state
- Line 172: aseek async method for file positioning
- Line 201: awrite async method for writing data
- Line 205: close() idempotency and closed state handling
- Lines 220, 231-235: file property and as_bytes_io() position preservation
"""

from __future__ import annotations

import io
import tempfile

import pytest

from spikard.datastructures import UploadFile


class TestUploadFileRolledToDisk:
    """Test the rolled_to_disk property that checks SpooledTemporaryFile state."""

    def test_small_file_stays_in_memory(self) -> None:
        """Test that small files stay in memory (rolled_to_disk = False)."""
        content = b"small content"
        upload = UploadFile(
            filename="small.txt",
            content=content,
            max_spool_size=1024 * 1024,  # 1MB threshold
        )

        assert upload.rolled_to_disk is False

    def test_large_file_spills_to_disk(self) -> None:
        """Test that large files spill to disk (rolled_to_disk = True)."""
        # Create file larger than 1KB threshold
        large_content = b"x" * (2 * 1024)  # 2KB
        upload = UploadFile(
            filename="large.bin",
            content=large_content,
            max_spool_size=1024,  # 1KB threshold
        )

        assert upload.rolled_to_disk is True

    def test_file_at_threshold_stays_in_memory(self) -> None:
        """Test file exactly at threshold remains in memory."""
        threshold = 1024
        content = b"x" * threshold  # Exactly at threshold
        upload = UploadFile(
            filename="threshold.bin",
            content=content,
            max_spool_size=threshold,
        )

        # At threshold should stay in memory
        assert upload.rolled_to_disk is False

    def test_file_one_byte_over_threshold_rolls_to_disk(self) -> None:
        """Test file one byte over threshold rolls to disk."""
        threshold = 1024
        content = b"x" * (threshold + 1)
        upload = UploadFile(
            filename="over.bin",
            content=content,
            max_spool_size=threshold,
        )

        assert upload.rolled_to_disk is True

    def test_empty_file_stays_in_memory(self) -> None:
        """Test that empty files stay in memory."""
        upload = UploadFile(
            filename="empty.txt",
            content=b"",
            max_spool_size=1024,
        )

        assert upload.rolled_to_disk is False

    def test_rolled_to_disk_after_write_exceeding_threshold(self) -> None:
        """Test rolled_to_disk changes after writing beyond threshold."""
        threshold = 100
        upload = UploadFile(
            filename="test.bin",
            content=b"small",
            max_spool_size=threshold,
        )

        # Initially small
        assert upload.rolled_to_disk is False

        # Write more to exceed threshold
        upload.seek(0, 2)  # Seek to end
        upload.write(b"x" * 200)

        # Should now be on disk
        assert upload.rolled_to_disk is True

    def test_rolled_to_disk_false_attribute_access(self) -> None:
        """Test accessing _rolled attribute directly returns False when not present."""
        upload = UploadFile(
            filename="test.txt",
            content=b"content",
            max_spool_size=1024,
        )

        # getattr with default should return False if _rolled not set
        rolled = getattr(upload._file, "_rolled", False)
        assert rolled is False
        assert upload.rolled_to_disk is False


class TestUploadFileAsync:
    """Test async methods (aseek, awrite) for file operations."""

    @pytest.mark.asyncio
    async def test_aseek_to_start(self) -> None:
        """Test async seek to start of file."""
        content = b"test content"
        upload = UploadFile(filename="test.txt", content=content)

        # Seek somewhere
        upload.seek(5)
        # Use aseek to go back to start
        result = await upload.aseek(0)
        assert result == 0
        assert upload._file.tell() == 0

    @pytest.mark.asyncio
    async def test_aseek_to_end(self) -> None:
        """Test async seek to end of file."""
        content = b"test content"
        upload = UploadFile(filename="test.txt", content=content)

        result = await upload.aseek(0, 2)  # whence=2 is SEEK_END
        assert result == len(content)
        assert upload._file.tell() == len(content)

    @pytest.mark.asyncio
    async def test_aseek_relative(self) -> None:
        """Test async seek with relative positioning (whence=1)."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        upload.seek(0)
        # Seek 5 bytes forward from current position
        result = await upload.aseek(5, 1)
        assert result == 5
        assert upload._file.tell() == 5

    @pytest.mark.asyncio
    async def test_aseek_from_end_negative_offset(self) -> None:
        """Test async seek from end with negative offset."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Seek -5 bytes from end
        await upload.aseek(-5, 2)
        assert upload._file.tell() == len(content) - 5

    @pytest.mark.asyncio
    async def test_aseek_backward_relative(self) -> None:
        """Test async seek backward with relative positioning."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        upload.seek(8)
        # Seek back 3 bytes from current
        result = await upload.aseek(-3, 1)
        assert result == 5
        assert upload._file.tell() == 5

    @pytest.mark.asyncio
    async def test_aseek_multiple_times(self) -> None:
        """Test multiple async seeks in sequence."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        pos1 = await upload.aseek(2)
        assert pos1 == 2

        pos2 = await upload.aseek(5)
        assert pos2 == 5

        pos3 = await upload.aseek(0)
        assert pos3 == 0

    @pytest.mark.asyncio
    async def test_awrite_single_chunk(self) -> None:
        """Test async write of a single chunk."""
        upload = UploadFile(filename="test.txt", content=b"")

        data = b"async written data"
        bytes_written = await upload.awrite(data)

        assert bytes_written == len(data)
        upload.seek(0)
        assert upload.read() == data

    @pytest.mark.asyncio
    async def test_awrite_multiple_chunks(self) -> None:
        """Test multiple async writes in sequence."""
        upload = UploadFile(filename="test.txt", content=b"initial")

        upload.seek(0, 2)  # Seek to end
        bytes1 = await upload.awrite(b"chunk1")
        bytes2 = await upload.awrite(b"chunk2")

        assert bytes1 == 6
        assert bytes2 == 6

        upload.seek(0)
        assert upload.read() == b"initialchunk1chunk2"

    @pytest.mark.asyncio
    async def test_awrite_empty_bytes(self) -> None:
        """Test async write of empty bytes."""
        upload = UploadFile(filename="test.txt", content=b"original")

        bytes_written = await upload.awrite(b"")
        assert bytes_written == 0

        upload.seek(0)
        assert upload.read() == b"original"

    @pytest.mark.asyncio
    async def test_awrite_large_data(self) -> None:
        """Test async write of large data."""
        upload = UploadFile(filename="test.bin", content=b"")

        large_data = b"x" * (1024 * 1024)  # 1MB
        bytes_written = await upload.awrite(large_data)

        assert bytes_written == len(large_data)
        assert upload.size >= len(large_data)

    @pytest.mark.asyncio
    async def test_awrite_updates_size(self) -> None:
        """Test that awrite updates the size property."""
        upload = UploadFile(filename="test.txt", content=b"initial")
        initial_size = upload.size

        upload.seek(0, 2)  # Seek to end
        await upload.awrite(b"more data")

        assert upload.size > initial_size

    @pytest.mark.asyncio
    async def test_awrite_and_aseek_combined(self) -> None:
        """Test combining awrite and aseek operations."""
        upload = UploadFile(filename="test.txt", content=b"")

        # Write first part
        await upload.awrite(b"Hello ")
        # Seek back to add in middle
        await upload.aseek(6)
        # Append
        await upload.awrite(b"World!")

        upload.seek(0)
        assert upload.read() == b"Hello World!"


class TestUploadFileClosing:
    """Test file closing operations and idempotency."""

    def test_close_once(self) -> None:
        """Test closing file works correctly."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()
        assert upload._file.closed is True

    def test_close_twice_idempotent(self) -> None:
        """Test that closing twice doesn't raise an exception."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()
        # Close again - should be safe
        upload.close()
        # No exception should be raised
        assert upload._file.closed is True

    def test_close_three_times(self) -> None:
        """Test closing multiple times is safe."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()
        upload.close()
        upload.close()
        assert upload._file.closed is True

    def test_close_guards_already_closed(self) -> None:
        """Test the if not self._file.closed guard works."""
        upload = UploadFile(filename="test.txt", content=b"content")

        # Manually close the underlying file
        upload._file.close()

        # Now call close() - should not raise
        upload.close()
        assert upload._file.closed is True

    def test_operations_after_close_raise(self) -> None:
        """Test that file operations after close raise ValueError."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()

        # Reading from closed file should raise
        with pytest.raises(ValueError, match="I/O operation on closed file"):
            upload.read()

    def test_seek_after_close_raises(self) -> None:
        """Test that seek after close raises ValueError."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()

        with pytest.raises(ValueError, match="I/O operation on closed file"):
            upload.seek(0)

    def test_write_after_close_raises(self) -> None:
        """Test that write after close raises ValueError."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()

        with pytest.raises(ValueError, match="I/O operation on closed file"):
            upload.write(b"more")

    @pytest.mark.asyncio
    async def test_aclose_idempotent(self) -> None:
        """Test that async close is also idempotent."""
        upload = UploadFile(filename="test.txt", content=b"content")

        await upload.aclose()
        await upload.aclose()
        assert upload._file.closed is True


class TestUploadFileProperties:
    """Test file property and as_bytes_io() behavior."""

    def test_file_property_returns_underlying_file(self) -> None:
        """Test that .file property returns SpooledTemporaryFile."""
        content = b"test content"
        upload = UploadFile(filename="test.txt", content=content)

        returned_file = upload.file
        assert returned_file is upload._file
        assert isinstance(returned_file, tempfile.SpooledTemporaryFile)

    def test_file_property_is_readable(self) -> None:
        """Test that file property can be used directly."""
        content = b"content via property"
        upload = UploadFile(filename="test.txt", content=content)

        # Use property to read
        upload.file.seek(0)
        data = upload.file.read()
        assert data == content

    def test_file_property_is_writable(self) -> None:
        """Test that file property can be written to."""
        upload = UploadFile(filename="test.txt", content=b"initial")

        upload.file.seek(0, 2)  # Seek to end
        upload.file.write(b" appended")

        upload.file.seek(0)
        assert upload.file.read() == b"initial appended"

    def test_as_bytes_io_preserves_position(self) -> None:
        """Test as_bytes_io() preserves original file position."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Position at byte 5
        upload.seek(5)
        original_pos = upload._file.tell()

        # Get BytesIO
        _ = upload.as_bytes_io()

        # Original position should be preserved
        assert upload._file.tell() == original_pos
        assert upload._file.tell() == 5

    def test_as_bytes_io_contains_full_content(self) -> None:
        """Test as_bytes_io() contains all file data."""
        content = b"full content here"
        upload = UploadFile(filename="test.txt", content=content)

        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)

        assert bytes_io.read() == content

    def test_as_bytes_io_returns_bytes_io_instance(self) -> None:
        """Test as_bytes_io() returns io.BytesIO instance."""
        upload = UploadFile(filename="test.txt", content=b"content")

        bytes_io = upload.as_bytes_io()
        assert isinstance(bytes_io, io.BytesIO)

    def test_as_bytes_io_independent_of_original(self) -> None:
        """Test BytesIO modifications don't affect original."""
        upload = UploadFile(filename="test.txt", content=b"original")

        original_pos = upload._file.tell()

        # Get BytesIO and modify its position
        bytes_io = upload.as_bytes_io()
        bytes_io.seek(5)

        # Original upload position unchanged
        assert upload._file.tell() == original_pos

    def test_as_bytes_io_from_middle_position(self) -> None:
        """Test as_bytes_io() when file position is in middle."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Position in middle
        upload.seek(3)

        # Get BytesIO - should contain full content
        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)
        assert bytes_io.read() == content

        # Original still at position 3
        assert upload._file.tell() == 3

    def test_as_bytes_io_from_end_position(self) -> None:
        """Test as_bytes_io() when at end of file."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Position at end
        upload.seek(0, 2)

        # Get BytesIO - should still have full content
        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)
        assert bytes_io.read() == content

        # Original still at end
        assert upload._file.tell() == len(content)

    def test_as_bytes_io_empty_file(self) -> None:
        """Test as_bytes_io() with empty file."""
        upload = UploadFile(filename="empty.txt", content=b"")

        bytes_io = upload.as_bytes_io()
        assert bytes_io.read() == b""

    def test_as_bytes_io_large_file(self) -> None:
        """Test as_bytes_io() with large file content."""
        content = b"x" * (10 * 1024)  # 10KB
        upload = UploadFile(filename="large.bin", content=content)

        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)
        assert bytes_io.read() == content

    def test_as_bytes_io_multiple_calls(self) -> None:
        """Test multiple as_bytes_io() calls return independent instances."""
        content = b"shared content"
        upload = UploadFile(filename="test.txt", content=content)

        bytes_io1 = upload.as_bytes_io()
        bytes_io2 = upload.as_bytes_io()

        # They should be different objects
        assert bytes_io1 is not bytes_io2

        # But contain same data
        bytes_io1.seek(0)
        bytes_io2.seek(0)
        assert bytes_io1.read() == bytes_io2.read()

    def test_as_bytes_io_position_after_partial_read(self) -> None:
        """Test as_bytes_io() after partial reads."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Partial reads
        upload.read(3)
        upload.read(2)
        current_pos = upload._file.tell()

        # Get BytesIO
        bytes_io = upload.as_bytes_io()

        # Original position preserved
        assert upload._file.tell() == current_pos
        assert upload._file.tell() == 5

        # BytesIO has full content
        bytes_io.seek(0)
        assert bytes_io.read() == content


class TestUploadFileIntegration:
    """Integration tests combining multiple operations."""

    def test_write_then_as_bytes_io(self) -> None:
        """Test as_bytes_io() after write operations."""
        upload = UploadFile(filename="test.txt", content=b"initial")

        upload.seek(0, 2)
        upload.write(b" more")

        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)
        assert bytes_io.read() == b"initial more"

    def test_seek_read_write_cycle(self) -> None:
        """Test complete seek/read/write cycle."""
        upload = UploadFile(filename="test.txt", content=b"0123456789")

        # Seek to position 5
        upload.seek(5)
        # Read 3 bytes
        data = upload.read(3)
        assert data == b"567"

        # Write at end
        upload.seek(0, 2)
        upload.write(b"!")

        # Verify full content
        upload.seek(0)
        assert upload.read() == b"0123456789!"

    @pytest.mark.asyncio
    async def test_async_operations_sequence(self) -> None:
        """Test sequence of async operations."""
        upload = UploadFile(filename="test.txt", content=b"start")

        # Async seek to end
        await upload.aseek(0, 2)
        # Async write
        await upload.awrite(b" middle")
        # Async seek back to start
        await upload.aseek(0)
        # Async read
        data = await upload.aread()

        assert data == b"start middle"

    def test_context_manager_with_operations(self) -> None:
        """Test context manager ensures proper cleanup."""
        with UploadFile(filename="test.txt", content=b"content") as upload:
            data = upload.read()
            assert data == b"content"

        # Should be closed after context exit
        assert upload._file.closed is True

    @pytest.mark.asyncio
    async def test_async_context_manager_with_operations(self) -> None:
        """Test async context manager ensures proper cleanup."""
        async with UploadFile(filename="test.txt", content=b"content") as upload:
            data = await upload.aread()
            assert data == b"content"

        # Should be closed after context exit
        assert upload._file.closed is True

    def test_size_tracking_through_writes(self) -> None:
        """Test that size property is correctly updated through writes."""
        upload = UploadFile(filename="test.txt", content=b"initial")
        initial_size = upload.size
        assert initial_size == 7

        upload.seek(0, 2)
        upload.write(b"more data")

        # Size should be updated
        assert upload.size == 7 + 9

    def test_headers_and_metadata(self) -> None:
        """Test headers and metadata properties."""
        headers = {"X-Custom": "value"}
        upload = UploadFile(
            filename="test.txt",
            content=b"content",
            content_type="text/custom",
            headers=headers,
        )

        assert upload.headers == headers
        assert upload.content_type == "text/custom"
        assert upload.filename == "test.txt"

    def test_default_headers_empty_dict(self) -> None:
        """Test that default headers is empty dict."""
        upload = UploadFile(filename="test.txt", content=b"content")

        assert upload.headers == {}
        assert isinstance(upload.headers, dict)

    def test_default_content_type(self) -> None:
        """Test default content type."""
        upload = UploadFile(filename="test.bin", content=b"data")

        assert upload.content_type == "application/octet-stream"

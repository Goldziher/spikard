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
    async def test_aseek_parametrized(self) -> None:
        """Test async seek with various whence positions."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Test seek to start
        result = await upload.aseek(0)
        assert result == 0
        assert upload._file.tell() == 0

        # Test seek to end
        result = await upload.aseek(0, 2)
        assert result == len(content)
        assert upload._file.tell() == len(content)

        # Test relative seek
        upload.seek(5)
        result = await upload.aseek(3, 1)
        assert result == 8
        assert upload._file.tell() == 8

    @pytest.mark.asyncio
    async def test_awrite_variants(self) -> None:
        """Test async write with various data patterns."""
        upload = UploadFile(filename="test.txt", content=b"initial")

        # Single write
        upload.seek(0, 2)
        bytes_written = await upload.awrite(b"chunk1")
        assert bytes_written == 6

        # Empty write
        bytes_written = await upload.awrite(b"")
        assert bytes_written == 0

        # Verify result
        upload.seek(0)
        assert upload.read() == b"initialchunk1"


class TestUploadFileClosing:
    """Test file closing operations and idempotency."""

    def test_close_idempotent(self) -> None:
        """Test that closing multiple times is safe."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()
        upload.close()
        assert upload._file.closed is True

    def test_operations_after_close_raise(self) -> None:
        """Test that file operations after close raise ValueError."""
        upload = UploadFile(filename="test.txt", content=b"content")

        upload.close()

        with pytest.raises(ValueError, match="I/O operation on closed file"):
            upload.read()

    @pytest.mark.asyncio
    async def test_aclose_idempotent(self) -> None:
        """Test that async close is also idempotent."""
        upload = UploadFile(filename="test.txt", content=b"content")

        await upload.aclose()
        await upload.aclose()
        assert upload._file.closed is True


class TestUploadFileProperties:
    """Test file property and as_bytes_io() behavior."""

    def test_as_bytes_io_preserves_position_and_content(self) -> None:
        """Test as_bytes_io() preserves position and contains full content."""
        content = b"0123456789"
        upload = UploadFile(filename="test.txt", content=content)

        # Position at byte 5
        upload.seek(5)

        # Get BytesIO
        bytes_io = upload.as_bytes_io()
        bytes_io.seek(0)

        # Original position preserved
        assert upload._file.tell() == 5
        # BytesIO has full content
        assert bytes_io.read() == content

    def test_as_bytes_io_returns_independent_instance(self) -> None:
        """Test as_bytes_io() returns independent BytesIO instance."""
        content = b"shared content"
        upload = UploadFile(filename="test.txt", content=content)

        bytes_io1 = upload.as_bytes_io()
        bytes_io2 = upload.as_bytes_io()

        assert bytes_io1 is not bytes_io2
        assert isinstance(bytes_io1, io.BytesIO)

        bytes_io1.seek(0)
        bytes_io2.seek(0)
        assert bytes_io1.read() == bytes_io2.read()


class TestUploadFileIntegration:
    """Integration tests combining multiple operations."""

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

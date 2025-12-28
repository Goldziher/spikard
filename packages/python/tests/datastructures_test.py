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

# TestUploadFileRolledToDisk tests


def test_upload_file_rolled_to_disk_small_file_stays_in_memory() -> None:
    """Test that small files stay in memory (rolled_to_disk = False)."""
    content = b"small content"
    upload = UploadFile(
        filename="small.txt",
        content=content,
        max_spool_size=1024 * 1024,
    )

    assert upload.rolled_to_disk is False


def test_upload_file_rolled_to_disk_large_file_spills_to_disk() -> None:
    """Test that large files spill to disk (rolled_to_disk = True)."""
    large_content = b"x" * (2 * 1024)
    upload = UploadFile(
        filename="large.bin",
        content=large_content,
        max_spool_size=1024,
    )

    assert upload.rolled_to_disk is True


def test_upload_file_rolled_to_disk_file_at_threshold_stays_in_memory() -> None:
    """Test file exactly at threshold remains in memory."""
    threshold = 1024
    content = b"x" * threshold
    upload = UploadFile(
        filename="threshold.bin",
        content=content,
        max_spool_size=threshold,
    )

    assert upload.rolled_to_disk is False


def test_upload_file_rolled_to_disk_file_one_byte_over_threshold_rolls_to_disk() -> None:
    """Test file one byte over threshold rolls to disk."""
    threshold = 1024
    content = b"x" * (threshold + 1)
    upload = UploadFile(
        filename="over.bin",
        content=content,
        max_spool_size=threshold,
    )

    assert upload.rolled_to_disk is True


def test_upload_file_rolled_to_disk_empty_file_stays_in_memory() -> None:
    """Test that empty files stay in memory."""
    upload = UploadFile(
        filename="empty.txt",
        content=b"",
        max_spool_size=1024,
    )

    assert upload.rolled_to_disk is False


def test_upload_file_rolled_to_disk_rolled_to_disk_after_write_exceeding_threshold() -> None:
    """Test rolled_to_disk changes after writing beyond threshold."""
    threshold = 100
    upload = UploadFile(
        filename="test.bin",
        content=b"small",
        max_spool_size=threshold,
    )

    assert upload.rolled_to_disk is False

    upload.seek(0, 2)
    upload.write(b"x" * 200)

    assert upload.rolled_to_disk is True


def test_upload_file_rolled_to_disk_rolled_to_disk_false_attribute_access() -> None:
    """Test accessing _rolled attribute directly returns False when not present."""
    upload = UploadFile(
        filename="test.txt",
        content=b"content",
        max_spool_size=1024,
    )

    rolled = getattr(upload._file, "_rolled", False)
    assert rolled is False
    assert upload.rolled_to_disk is False


# TestUploadFileAsync tests


@pytest.mark.asyncio
async def test_upload_file_async_aseek_parametrized() -> None:
    """Test async seek with various whence positions."""
    content = b"0123456789"
    upload = UploadFile(filename="test.txt", content=content)

    result = await upload.aseek(0)
    assert result == 0
    assert upload._file.tell() == 0

    result = await upload.aseek(0, 2)
    assert result == len(content)
    assert upload._file.tell() == len(content)

    upload.seek(5)
    result = await upload.aseek(3, 1)
    assert result == 8
    assert upload._file.tell() == 8


@pytest.mark.asyncio
async def test_upload_file_async_awrite_variants() -> None:
    """Test async write with various data patterns."""
    upload = UploadFile(filename="test.txt", content=b"initial")

    upload.seek(0, 2)
    bytes_written = await upload.awrite(b"chunk1")
    assert bytes_written == 6

    bytes_written = await upload.awrite(b"")
    assert bytes_written == 0

    upload.seek(0)
    assert upload.read() == b"initialchunk1"


# TestUploadFileClosing tests


def test_upload_file_closing_close_idempotent() -> None:
    """Test that closing multiple times is safe."""
    upload = UploadFile(filename="test.txt", content=b"content")

    upload.close()
    upload.close()
    assert upload._file.closed is True


def test_upload_file_closing_operations_after_close_raise() -> None:
    """Test that file operations after close raise ValueError."""
    upload = UploadFile(filename="test.txt", content=b"content")

    upload.close()

    with pytest.raises(ValueError, match="I/O operation on closed file"):
        upload.read()


@pytest.mark.asyncio
async def test_upload_file_closing_aclose_idempotent() -> None:
    """Test that async close is also idempotent."""
    upload = UploadFile(filename="test.txt", content=b"content")

    await upload.aclose()
    await upload.aclose()
    assert upload._file.closed is True


# TestUploadFileProperties tests


def test_upload_file_properties_as_bytes_io_preserves_position_and_content() -> None:
    """Test as_bytes_io() preserves position and contains full content."""
    content = b"0123456789"
    upload = UploadFile(filename="test.txt", content=content)

    upload.seek(5)

    bytes_io = upload.as_bytes_io()
    bytes_io.seek(0)

    assert upload._file.tell() == 5
    assert bytes_io.read() == content


def test_upload_file_properties_as_bytes_io_returns_independent_instance() -> None:
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


# TestUploadFileIntegration tests


def test_upload_file_integration_seek_read_write_cycle() -> None:
    """Test complete seek/read/write cycle."""
    upload = UploadFile(filename="test.txt", content=b"0123456789")

    upload.seek(5)
    data = upload.read(3)
    assert data == b"567"

    upload.seek(0, 2)
    upload.write(b"!")

    upload.seek(0)
    assert upload.read() == b"0123456789!"


@pytest.mark.asyncio
async def test_upload_file_integration_async_operations_sequence() -> None:
    """Test sequence of async operations."""
    upload = UploadFile(filename="test.txt", content=b"start")

    await upload.aseek(0, 2)
    await upload.awrite(b" middle")
    await upload.aseek(0)
    data = await upload.aread()

    assert data == b"start middle"


def test_upload_file_integration_context_manager_with_operations() -> None:
    """Test context manager ensures proper cleanup."""
    with UploadFile(filename="test.txt", content=b"content") as upload:
        data = upload.read()
        assert data == b"content"

    assert upload._file.closed is True

"""E2E tests for streaming."""

from spikard.testing import TestClient
from app.main import (
    create_app_streaming_binary_log_download,
    create_app_streaming_chunked_csv_export,
    create_app_streaming_stream_json_lines,
)


async def test_stream_json_lines() -> None:
    """Streams newline-delimited JSON payload in small chunks."""

    app = create_app_streaming_stream_json_lines()
    client = TestClient(app)

    response = await client.get("/stream/json-lines")

    assert response.status_code == 200
    expected_bytes = b'{"index":0,"payload":"alpha"}\\n{"index":1,"payload":"beta"}\\n{"index":2,"payload":"gamma"}\\n'
    assert response.bytes() == expected_bytes
    assert response.text() == expected_bytes.decode()


async def test_binary_log_download() -> None:
    """Streams binary log segments with control bytes."""

    app = create_app_streaming_binary_log_download()
    client = TestClient(app)

    response = await client.get("/stream/logfile")

    assert response.status_code == 200
    expected_bytes = b"LOG:\x00\x01\x02\x03|TAIL|\x07\\n"
    assert response.bytes() == expected_bytes


async def test_chunked_csv_export() -> None:
    """Streams CSV header and rows as discrete chunks."""

    app = create_app_streaming_chunked_csv_export()
    client = TestClient(app)

    response = await client.get("/stream/csv-report")

    assert response.status_code == 200
    expected_bytes = b"id,name,value\\n1,Alice,42\\n2,Bob,7\\n"
    assert response.bytes() == expected_bytes
    assert response.text() == expected_bytes.decode()

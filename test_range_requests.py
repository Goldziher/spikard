"""Test to verify HTTP Range requests work with Spikard's static file serving."""
# ruff: noqa: PLC0415, S101

import asyncio
import tempfile
from pathlib import Path


async def test_range_request() -> None:
    """Verify that Spikard supports HTTP Range requests via tower-http ServeDir."""
    try:
        # Import after path setup
        import sys

        sys.path.insert(0, str(Path(__file__).parent / "packages" / "python"))

        from spikard import ServerConfig, Spikard, StaticFilesConfig
        from spikard.testing import TestClient

        # Create temporary directory with a test file
        with tempfile.TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "document.pdf"
            # Create a 5000-byte file
            test_content = b"X" * 5000
            test_file.write_bytes(test_content)

            # Configure server with static files
            config = ServerConfig(static_files=[StaticFilesConfig(directory=tmpdir, route_prefix="/files")])

            app = Spikard(config=config)

            async with TestClient(app) as client:
                # Test 1: Request bytes 0-1023 (first 1024 bytes)
                response = await client.get("/files/document.pdf", headers={"Range": "bytes=0-1023"})

                assert response.status_code == 206, f"Expected 206, got {response.status_code}"
                assert response.headers.get("content-range") == "bytes 0-1023/5000"
                assert response.headers.get("content-length") == "1024"
                assert response.headers.get("accept-ranges") == "bytes"
                assert len(response.content) == 1024
                assert response.content == test_content[0:1024]

                # Test 2: Request last 500 bytes
                response = await client.get("/files/document.pdf", headers={"Range": "bytes=-500"})

                assert response.status_code == 206
                assert len(response.content) == 500
                assert response.content == test_content[-500:]

                # Test 3: Request from byte 2000 to end
                response = await client.get("/files/document.pdf", headers={"Range": "bytes=2000-"})

                assert response.status_code == 206
                assert len(response.content) == 3000  # 5000 - 2000
                assert response.content == test_content[2000:]

                # Test 4: Request without Range header (should return 200)
                response = await client.get("/files/document.pdf")

                assert response.status_code == 200
                assert len(response.content) == 5000
                assert response.content == test_content

    except Exception:
        import traceback

        traceback.print_exc()
        raise


if __name__ == "__main__":
    asyncio.run(test_range_request())

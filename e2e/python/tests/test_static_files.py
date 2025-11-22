"""E2E tests for static_files."""

from spikard.testing import TestClient
from app.main import (
    create_app_static_files_static_file_server_returns_text_file,
    create_app_static_files_static_server_returns_index_html_for_directory,
)


async def test_static_file_server_returns_text_file() -> None:
    """Tests serving a plain text file with Cache-Control headers.."""

    async with TestClient(create_app_static_files_static_file_server_returns_text_file()) as client:
        response = await client.get("/public/hello.txt")

        assert response.status_code == 200
        assert response.text == "Hello from static storage\n"
        response_headers = response.headers
        assert response_headers.get("content-type") == "text/plain"
        assert response_headers.get("cache-control") == "public, max-age=60"


async def test_static_server_returns_index_html_for_directory() -> None:
    """When index files are enabled the server should serve index.html when the directory root is requested.."""

    async with TestClient(create_app_static_files_static_server_returns_index_html_for_directory()) as client:
        response = await client.get("/app/")

        assert response.status_code == 200
        assert response.text == "<!doctype html><h1>Welcome</h1>\n"
        response_headers = response.headers
        assert response_headers.get("content-type") == "text/html"

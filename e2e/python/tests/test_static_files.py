"""E2E tests for static_files."""

from spikard.testing import TestClient
from app.main import (
    create_app_static_files_304_not_modified_with_complete_cache_headers,
    create_app_static_files_403_forbidden_for_path_traversal_attempt,
    create_app_static_files_404_not_found_for_missing_file,
    create_app_static_files_cache_control_with_max_age_for_static_assets,
    create_app_static_files_custom_index_file_name,
    create_app_static_files_directory_request_serves_index_html,
    create_app_static_files_etag_validation_with_if_none_match,
    create_app_static_files_last_modified_validation_with_if_modified_since,
    create_app_static_files_multiple_ranges_not_supported,
    create_app_static_files_partial_content_range_request,
    create_app_static_files_serve_css_file,
    create_app_static_files_serve_html_file,
    create_app_static_files_serve_javascript_file,
    create_app_static_files_serve_json_file,
    create_app_static_files_serve_png_image,
    create_app_static_files_serve_pre_compressed_brotli_file,
    create_app_static_files_serve_pre_compressed_gzip_file,
    create_app_static_files_spa_fallback_for_client_side_routing,
    create_app_static_files_static_files_disabled_returns_404,
    create_app_static_files_unknown_mime_type_fallback,
)


async def test_static_files_disabled_returns_404() -> None:
    """Tests that when static files middleware is null/disabled, requests return 404."""

    app = create_app_static_files_static_files_disabled_returns_404()
    client = TestClient(app)

    response = await client.get("/styles/main.css")

    assert response.status_code == 404
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "No route matched the request path"
    assert "status" in response_data
    assert response_data["status"] == 404
    assert "title" in response_data
    assert response_data["title"] == "Not Found"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/not-found"


async def test_multiple_ranges_not_supported() -> None:
    """Tests that multiple range requests return full content with 200 instead of 206."""

    app = create_app_static_files_multiple_ranges_not_supported()
    client = TestClient(app)

    headers = {
        "Range": "bytes=0-999, 2000-2999, 5000-5999",
    }
    response = await client.get("/documents/manual.pdf", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "documents/manual.pdf"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 524288


async def test_etag_validation_with_if_none_match() -> None:
    """Tests 304 Not Modified response when ETag matches If-None-Match header."""

    app = create_app_static_files_etag_validation_with_if_none_match()
    client = TestClient(app)

    headers = {
        "If-None-Match": '"abc123"',
    }
    response = await client.get("/about.html", headers=headers)

    assert response.status_code == 304


async def test_unknown_mime_type_fallback() -> None:
    """Tests fallback to application/octet-stream for files with unknown extensions."""

    app = create_app_static_files_unknown_mime_type_fallback()
    client = TestClient(app)

    response = await client.get("/files/data.xyz")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "files/data.xyz"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 4096


async def test_404_not_found_for_missing_file() -> None:
    """Tests 404 response with RFC 7807 problem details when requested file does not exist."""

    app = create_app_static_files_404_not_found_for_missing_file()
    client = TestClient(app)

    response = await client.get("/missing.html")

    assert response.status_code == 404
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "The requested file '/missing.html' was not found"
    assert "status" in response_data
    assert response_data["status"] == 404
    assert "title" in response_data
    assert response_data["title"] == "Not Found"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/not-found"


async def test_serve_json_file() -> None:
    """Tests serving static JSON data file with correct Content-Type."""

    app = create_app_static_files_serve_json_file()
    client = TestClient(app)

    response = await client.get("/data/config.json")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_content_sample" in response_data
    assert response_data["simulated_content_sample"] == '{"version": "1.0.0"}'
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "data/config.json"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 1024


async def test_serve_pre_compressed_gzip_file() -> None:
    """Tests serving .gz pre-compressed file when client accepts gzip encoding."""

    app = create_app_static_files_serve_pre_compressed_gzip_file()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "gzip, deflate, br",
    }
    response = await client.get("/js/app.js", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "js/app.js.gz"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 2048


async def test_last_modified_validation_with_if_modified_since() -> None:
    """Tests 304 Not Modified response when file has not been modified since If-Modified-Since timestamp."""

    app = create_app_static_files_last_modified_validation_with_if_modified_since()
    client = TestClient(app)

    headers = {
        "If-Modified-Since": "Wed, 21 Oct 2025 07:28:00 GMT",
    }
    response = await client.get("/styles/main.css", headers=headers)

    assert response.status_code == 304


async def test_serve_javascript_file() -> None:
    """Tests serving static JavaScript module with correct Content-Type."""

    app = create_app_static_files_serve_javascript_file()
    client = TestClient(app)

    response = await client.get("/js/app.js")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_content_sample" in response_data
    assert response_data["simulated_content_sample"] == "console.log('Hello, world!');"
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "js/app.js"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 8192


async def test_serve_pre_compressed_brotli_file() -> None:
    """Tests serving .br pre-compressed file when client accepts Brotli encoding."""

    app = create_app_static_files_serve_pre_compressed_brotli_file()
    client = TestClient(app)

    headers = {
        "Accept-Encoding": "br, gzip, deflate",
    }
    response = await client.get("/js/app.js", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "js/app.js.br"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 1536


async def test_directory_request_serves_index_html() -> None:
    """Tests automatic index file serving when requesting a directory path."""

    app = create_app_static_files_directory_request_serves_index_html()
    client = TestClient(app)

    response = await client.get("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "index.html"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 3072


async def test_serve_css_file() -> None:
    """Tests serving static CSS stylesheet with correct Content-Type."""

    app = create_app_static_files_serve_css_file()
    client = TestClient(app)

    response = await client.get("/styles/main.css")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_content_sample" in response_data
    assert response_data["simulated_content_sample"] == "body { font-family: sans-serif; }"
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "styles/main.css"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 4096


async def test_403_forbidden_for_path_traversal_attempt() -> None:
    """Tests security protection against path traversal attacks using ../ sequences."""

    app = create_app_static_files_403_forbidden_for_path_traversal_attempt()
    client = TestClient(app)

    response = await client.get("/../../../etc/passwd")

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Path traversal is not allowed"
    assert "status" in response_data
    assert response_data["status"] == 403
    assert "title" in response_data
    assert response_data["title"] == "Forbidden"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/forbidden"


async def test_304_not_modified_with_complete_cache_headers() -> None:
    """Tests complete 304 response with ETag, Last-Modified, and Cache-Control headers."""

    app = create_app_static_files_304_not_modified_with_complete_cache_headers()
    client = TestClient(app)

    headers = {
        "If-None-Match": '"bcd890"',
        "If-Modified-Since": "Mon, 15 Oct 2025 10:00:00 GMT",
    }
    response = await client.get("/images/banner.jpg", headers=headers)

    assert response.status_code == 304


async def test_custom_index_file_name() -> None:
    """Tests serving custom index file (home.html) instead of default index.html."""

    app = create_app_static_files_custom_index_file_name()
    client = TestClient(app)

    response = await client.get("/")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "home.html"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 2560


async def test_cache_control_with_max_age_for_static_assets() -> None:
    """Tests proper Cache-Control header with max-age directive for long-lived assets."""

    app = create_app_static_files_cache_control_with_max_age_for_static_assets()
    client = TestClient(app)

    response = await client.get("/assets/bundle-v1.2.3.js")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "assets/bundle-v1.2.3.js"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 102400


async def test_spa_fallback_for_client_side_routing() -> None:
    """Tests fallback to index.html for missing files to support single-page application routing."""

    app = create_app_static_files_spa_fallback_for_client_side_routing()
    client = TestClient(app)

    response = await client.get("/dashboard/settings")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "index.html"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 3072


async def test_serve_html_file() -> None:
    """Tests serving static HTML file with correct Content-Type and caching headers."""

    app = create_app_static_files_serve_html_file()
    client = TestClient(app)

    response = await client.get("/about.html")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "about.html"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 2048


async def test_serve_png_image() -> None:
    """Tests serving binary image file with correct Content-Type and binary handling."""

    app = create_app_static_files_serve_png_image()
    client = TestClient(app)

    response = await client.get("/images/logo.png")

    assert response.status_code == 200
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "images/logo.png"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 15360


async def test_partial_content_range_request() -> None:
    """Tests HTTP Range request returning 206 Partial Content response."""

    app = create_app_static_files_partial_content_range_request()
    client = TestClient(app)

    headers = {
        "Range": "bytes=0-1023",
    }
    response = await client.get("/videos/demo.mp4", headers=headers)

    assert response.status_code == 206
    response_data = response.json()
    assert "simulated_file" in response_data
    assert response_data["simulated_file"] == "videos/demo.mp4"
    assert "simulated_size_bytes" in response_data
    assert response_data["simulated_size_bytes"] == 1024
